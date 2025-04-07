mod uv_tools;
mod i18n;

use std::path::Path;
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::env;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use dialoguer::{Select, theme::ColorfulTheme};
use console::style;
use walkdir::WalkDir;
use regex::Regex;
use indicatif::{ProgressBar, ProgressStyle};
use tempfile::tempdir;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

use crate::uv_tools::UvManager;
use crate::i18n::{I18n, Language};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 分析当前目录中的Python依赖
    Analyze {
        #[arg(short, long)]
        path: Option<String>,
    },
    /// 使用测试套件样本运行
    Test {
        #[arg(short, long, default_value = "test-suite")]
        path: String,
    },
    /// 直接执行本地开发流程
    LocalDev {
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// 直接生成requirements.txt文件
    GenReq {
        #[arg(short, long, default_value = ".")]
        path: String,
        
        #[arg(short, long, default_value = ".")]
        output: String,
    },
    /// 运行Python脚本
    Run {
        /// Python脚本路径
        script: String,
        
        /// 传递给脚本的参数
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// 直接执行uv命令
    Uv {
        /// uv子命令和参数
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// 安装Python包
    Pip {
        /// 要安装的包名
        #[arg(trailing_var_arg = true)]
        packages: Vec<String>,
    },
    /// 设置界面语言
    Lang {
        /// 语言代码：en, zh, ja, ko, fr, de, ru
        #[arg(short, long)]
        code: String,
    },
}

/// Python依赖分析和管理
struct PyWand {
    os_type: String,
    os_arch: String,
    python_files: Vec<String>,
    dependencies: Vec<String>,
    uv_manager: UvManager,
    internal_uv_path: Option<PathBuf>, // 内置uv工具的路径
    i18n: I18n, // 国际化支持
}

impl PyWand {
    /// 创建新的PyWand应用
    pub fn new() -> Self {
        // 尝试加载保存的语言设置，如果没有则使用系统语言
        let language = load_language_preference().unwrap_or_else(Language::default);
        let i18n = I18n::with_language(language);
        
        let os_type = determine_os_type();
        let os_arch = determine_os_arch();
        let mut app = PyWand {
            os_type,
            os_arch,
            python_files: Vec::new(),
            dependencies: Vec::new(),
            uv_manager: UvManager::new(),
            internal_uv_path: None,
            i18n,
        };
        
        // 确保内置的uv可用
        if let Err(e) = app.ensure_uv_available() {
            eprintln!("警告: 无法设置内置的uv工具: {}", e);
        }
        
        app
    }
    
    /// 确保内置的uv工具可用
    fn ensure_uv_available(&mut self) -> Result<()> {
        // 创建.pywand目录
        let pywand_dir = PathBuf::from(".pywand");
        fs::create_dir_all(&pywand_dir)
            .context("无法创建.pywand目录")?;
        
        // 确定uv文件名
        let uv_filename = if self.os_type == "windows" { "uv.exe" } else { "uv" };
        let uv_path = pywand_dir.join(uv_filename);
        
        // 检查uv是否已存在
        if !uv_path.exists() {
            println!("首次运行，正在设置内置uv工具...");
            
            // 从resources目录复制uv
            let resource_path = format!("resources/uv/{}-{}/{}", 
                self.os_type, self.os_arch, uv_filename);
                
            let resource_full_path = Path::new(&resource_path);
            if resource_full_path.exists() {
                fs::copy(resource_full_path, &uv_path)
                    .context(format!("无法复制uv从 {} 到 {}", resource_path, uv_path.display()))?;
                
                // 设置可执行权限(非Windows)
                if self.os_type != "windows" {
                    Command::new("chmod")
                        .args(["+x", uv_path.to_str().unwrap()])
                        .status()
                        .context("无法设置uv工具的执行权限")?;
                }
                
                println!("内置uv工具已设置完成！");
            } else {
                return Err(anyhow!("找不到适用于当前平台的uv工具: {}", resource_path));
            }
        }
        
        self.internal_uv_path = Some(uv_path);
        
        Ok(())
    }
    
    /// 获取内置uv工具的路径
    fn get_internal_uv_path(&self) -> Option<&Path> {
        self.internal_uv_path.as_ref().map(|p| p.as_path())
    }
    
    /// 应用程序主菜单
    fn show_main_menu(&mut self) -> Result<()> {
        println!("\n{}", style(self.i18n.get("app_name")).bold().cyan());
        println!("{}", style("=============================").bold().cyan());
        
        let options = vec![
            self.i18n.get("local_development"),
            self.i18n.get("export_offline"),
            self.i18n.get("exit")
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(self.i18n.get("what_to_do"))
            .default(0)
            .items(&options)
            .interact()?;
            
        match selection {
            0 => self.local_development_flow()?,
            1 => self.export_development_flow()?,
            2 => return Ok(()),
            _ => unreachable!(),
        }
        
        Ok(())
    }
    
    /// 本地开发设置
    fn local_development_flow(&mut self) -> Result<()> {
        println!("\n{}", style(self.i18n.get("local_dev_title")).bold().green());
        
        // 如果没有找到Python文件，提供选项
        if self.python_files.is_empty() {
            println!("{}", style(self.i18n.get("no_python_files")).bold().yellow());
            let options = vec![
                self.i18n.get("use_test_suite"),
                self.i18n.get("specify_directory"),
                self.i18n.get("cancel")
            ];
            
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(self.i18n.get("how_to_continue"))
                .default(0)
                .items(&options)
                .interact()?;
                
            match selection {
                0 => {
                    // 使用测试套件
                    println!("使用测试套件中的示例文件...");
                    self.find_python_files("test-suite")?;
                    if self.python_files.is_empty() {
                        println!("{}", style("测试套件中也未找到Python文件！").bold().red());
                        println!("请先创建一些Python文件，或使用'pywand test'命令运行测试套件。");
                        return Ok(());
                    }
                },
                1 => {
                    // 手动指定目录
                    let input = dialoguer::Input::<String>::new()
                        .with_prompt("请输入Python文件所在的目录路径")
                        .interact_text()?;
                    
                    self.find_python_files(&input)?;
                    if self.python_files.is_empty() {
                        println!("{}", style("指定目录中未找到Python文件！").bold().red());
                        return Ok(());
                    }
                },
                2 | _ => {
                    println!("操作已取消。");
                    return Ok(());
                }
            }
        }
        
        // 基于操作系统和UV支持选择Python版本
        let python_version = self.select_python_version()?;
        
        let creating_venv_msg = self.i18n.get_formatted(
            "creating_venv", 
            &[&python_version]
        );
        println!("\n{}", creating_venv_msg);
        
        // 确保UV可用
        self.uv_manager.ensure_available()?;
        
        // 创建虚拟环境
        let venv_dir = ".venv";
        self.uv_manager.create_venv(venv_dir, &python_version)?;
        
        // 生成requirements.txt文件到当前目录
        self.generate_requirements_file(".")?;
        
        // 安装依赖
        println!("{}", self.i18n.get("installing_dependencies"));
        self.uv_manager.install_dependencies("requirements.txt", venv_dir)?;
        
        // 创建激活脚本
        create_activation_scripts(venv_dir)?;
        
        println!("\n{}", style(self.i18n.get("setup_complete")).bold().green());
        println!("{}", self.i18n.get("to_activate_venv"));
        if cfg!(target_os = "windows") {
            println!("  .\\activate.bat");
        } else {
            println!("  source ./activate.sh");
        }
        
        // 添加使用提示
        show_usage_tips_with_language(self.i18n.language);
        
        Ok(())
    }
    
    /// 导出用于离线开发的设置
    fn export_development_flow(&mut self) -> Result<()> {
        println!("\n{}", style("导出用于离线开发").bold().green());
        
        // 操作系统选择
        let os_options = vec![
            "Windows 7 (32位)",
            "Windows 7 (64位)",
            "Windows 10 (32位)",
            "Windows 10 (64位)",
            "Windows 11 (64位)",
            "Windows Server (64位)"
        ];
        
        let os_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("选择目标操作系统")
            .default(3) // Windows 10 64位作为默认值
            .items(&os_options)
            .interact()?;
            
        // 基于所选操作系统选择Python版本
        let os_type = match os_selection {
            0 | 1 => "windows7",
            2 | 3 => "windows10", 
            4 => "windows11",
            5 => "windowsserver",
            _ => "windows10", // 默认
        };
        
        let arch = match os_selection {
            0 | 2 => "x86",
            _ => "x64",
        };
        
        let python_version = self.select_python_version_for_export(os_selection)?;
        
        println!("\n正在为{}和Python {}准备包...", 
                 os_options[os_selection], python_version);
                 
        // 如果self.python_files为空，那么我们需要扫描文件
        if self.python_files.is_empty() {
            self.find_python_files(".")?;
            self.extract_dependencies()?;
        }
        
        // 创建导出包
        let export_dir = tempdir()?;
        let export_path = export_dir.path();
        
        // 复制Python文件
        copy_python_files(&self.python_files, export_path)?;
        
        // 生成requirements.txt文件到导出目录
        self.generate_requirements_file(export_path.to_str().unwrap())?;
        
        // 为目标操作系统创建设置脚本
        create_setup_scripts(export_path, &python_version, os_type, arch)?;
        
        // 创建README文件
        create_readme(export_path, &python_version, &os_options[os_selection])?;
        
        // 创建zip存档
        let output_file = format!("pywand_export_{}_{}_{}.tar.gz", 
                                 os_type, arch, python_version.replace(".", "_"));
        create_archive(export_path, &output_file)?;
        
        println!("\n{}", style("导出成功完成！").bold().green());
        println!("包已保存到: ./{}", output_file);
        
        // 添加使用提示
        show_usage_tips_with_language(self.i18n.language);
        
        Ok(())
    }
    
    /// 在给定目录中查找所有Python文件
    fn find_python_files(&mut self, dir: &str) -> Result<()> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")?);
        pb.set_message("正在扫描Python文件...");
        
        self.python_files.clear(); // 清空之前的文件列表
        
        // 需要排除的目录名
        let excluded_dirs = [
            ".git", ".venv", "venv", "env", "__pycache__", "node_modules",
            ".idea", ".vscode", "dist", "build", "target", ".pytest_cache"
        ];
        
        for entry in WalkDir::new(dir)
            .max_depth(10) // 限制递归深度
            .into_iter()
            .filter_entry(|e| {
                // 排除特定目录
                if e.file_type().is_dir() {
                    let file_name = e.file_name().to_string_lossy();
                    return !excluded_dirs.iter().any(|d| &file_name == d);
                }
                true
            })
            .filter_map(|e| e.ok())
            .filter(|e| {
                if let Some(ext) = e.path().extension() {
                    ext == "py"
                } else {
                    false
                }
            }) 
        {
            self.python_files.push(entry.path().display().to_string());
            pb.tick();
        }
        
        let found_files_msg = format!("找到{}个Python文件", self.python_files.len());
        pb.finish_with_message(found_files_msg);
        
        println!("\n扫描目录: {}", dir);
        println!("找到Python文件数量: {}", self.python_files.len());
        
        Ok(())
    }
    
    /// 从Python文件中提取依赖
    fn extract_dependencies(&mut self) -> Result<()> {
        if self.python_files.is_empty() {
            println!("没有找到Python文件，无法提取依赖。");
            return Ok(());
        }
        
        let pb = ProgressBar::new(self.python_files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"));
        
        // 清空之前的依赖
        self.dependencies.clear();
        
        let import_re = Regex::new(r"(?m)^\s*(?:import|from)\s+([a-zA-Z0-9_]+)")?;
        
        for file in &self.python_files {
            if let Ok(content) = fs::read_to_string(file) {
                for cap in import_re.captures_iter(&content) {
                    let module = cap[1].to_string();
                    if !self.dependencies.contains(&module) 
                       && !is_standard_library(&module) {
                        self.dependencies.push(module);
                    }
                }
            }
            pb.inc(1);
        }
        
        pb.finish_with_message(format!("找到{}个依赖", self.dependencies.len()));
        
        // 显示依赖
        if !self.dependencies.is_empty() {
            println!("\n找到以下外部依赖：");
            for dep in &self.dependencies {
                println!("  - {}", dep);
            }
        } else {
            println!("\n未找到外部依赖。");
        }
        
        Ok(())
    }
    
    /// 基于操作系统和UV支持选择Python版本
    fn select_python_version(&self) -> Result<String> {
        let versions = get_supported_python_versions(&self.os_type, &self.os_arch);
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(self.i18n.get("select_python_version"))
            .default(0)
            .items(&versions)
            .interact()?;
            
        Ok(versions[selection].to_string())
    }
    
    /// 基于所选操作系统为导出选择Python版本
    fn select_python_version_for_export(&self, os_index: usize) -> Result<String> {
        let os_type = match os_index {
            0 | 1 => "windows7",
            2 | 3 => "windows10", 
            4 => "windows11",
            5 => "windowsserver",
            _ => "windows10", // 默认
        };
        
        let arch = match os_index {
            0 | 2 => "x86",
            _ => "x64",
        };
        
        let versions = get_supported_python_versions(os_type, arch);
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(self.i18n.get("select_python_version"))
            .default(0)
            .items(&versions)
            .interact()?;
            
        Ok(versions[selection].to_string())
    }
    
    /// 从提取的依赖生成requirements.txt文件
    fn generate_requirements_file(&self, target_dir: &str) -> Result<()> {
        let mut content = String::new();
        
        for dep in &self.dependencies {
            if let Some(normalized_dep) = normalize_package_name(dep) {
                content.push_str(&format!("{}\n", normalized_dep));
            }
        }
        
        let requirements_path = format!("{}/requirements.txt", target_dir.trim_end_matches('/'));
        
        fs::write(&requirements_path, content)
            .context(format!("无法写入{}文件", requirements_path))?;
            
        // 直接使用字符串格式化而不是i18n.get_formatted
        let req_created_msg = format!("创建了requirements.txt文件在 {}", target_dir);
        println!("{}", style(req_created_msg).bold().green());
        
        Ok(())
    }
}

/// 确定操作系统类型
fn determine_os_type() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else {
        "linux".to_string()
    }
}

/// 确定操作系统架构
fn determine_os_arch() -> String {
    if cfg!(target_arch = "x86_64") {
        "x64".to_string()
    } else if cfg!(target_arch = "x86") {
        "x86".to_string()
    } else if cfg!(target_arch = "aarch64") {
        "arm64".to_string()
    } else {
        "unknown".to_string()
    }
}

/// 检查模块是否是Python标准库的一部分
fn is_standard_library(module: &str) -> bool {
    // 扩展的Python标准库列表
    let std_libs = vec![
        "os", "sys", "re", "math", "json", "time", "datetime", "random", 
        "collections", "itertools", "functools", "pathlib", "subprocess",
        "typing", "abc", "argparse", "enum", "logging", "io", "csv",
        "__future__", "site", "threading", "importlib", "runpy", 
        "asyncio", "base64", "calendar", "contextlib", "copy", "dataclasses",
        "decimal", "difflib", "email", "hashlib", "html", "http", "inspect",
        "ipaddress", "multiprocessing", "operator", "platform", "pprint",
        "queue", "shutil", "signal", "socket", "sqlite3", "ssl", "statistics",
        "string", "struct", "tempfile", "textwrap", "unittest", "urllib",
        "uuid", "warnings", "xml", "zipfile", "zlib", "builtins", "codecs",
        "traceback", "pickle", "gzip", "array", "bisect", "configparser", 
        "context", "ctypes", "distutils", "fnmatch", "fractions", "ftplib",
        "getpass", "gettext", "glob", "heapq", "imp", "keyword", "marshal",
        "mimetypes", "numbers", "optparse", "posixpath", "profile", "pwd",
        "shelve", "smtplib", "symtable", "sysconfig", "tarfile", "telnetlib",
        "token", "turtle", "uu", "weakref", "winreg"
    ];
    
    std_libs.contains(&module)
}

/// 将模块名称转换为正确的PyPI包名或过滤掉无效的包名
fn normalize_package_name(module: &str) -> Option<String> {
    // 已知的PyPI包名映射
    let package_mappings = [
        ("yaml", "PyYAML"),
        ("PIL", "Pillow"),
        ("bs4", "beautifulsoup4"),
        ("sklearn", "scikit-learn"),
    ];
    
    // 返回已知映射的包名
    for (mod_name, pkg_name) in &package_mappings {
        if module == *mod_name {
            return Some(pkg_name.to_string());
        }
    }
    
    // 检查是否是无效的包名（单个字符、下划线开头等）
    if module.len() <= 1 || module.starts_with('_') || is_standard_library(module) ||
       ["name", "the", "header", "REPL", "code", "types", "stat", "line", "inline", 
        "another", "all", "values", "its", "regular", "each", "within", "working", 
        "source", "on", "what", "an", "multiple", "being", "that", "this", "inside", 
        "one", "floats", "those", "limited_api1", "limited_api_latest", "limited_api2", 
        "array_interface_testing", "mem_policy", "checks", "1", "0", "left", "lowest", 
        "pairs", "t2", "it", "outside", "running"].contains(&module) {
        return None;
    }
    
    // 返回原始模块名
    Some(module.to_string())
}

/// 获取给定操作系统和架构的UV支持的Python版本
fn get_supported_python_versions(os_type: &str, arch: &str) -> Vec<String> {
    // 理想情况下，这应该基于实际的UV文档/API
    // 目前，我们将根据操作系统和架构返回一个静态列表
    match (os_type, arch) {
        ("windows", "x64") | ("windows10", "x64") | ("windows11", "x64") => 
            vec!["3.8.10", "3.9.13", "3.10.11", "3.11.7", "3.12.1"].iter().map(|s| s.to_string()).collect(),
        ("windows", "x86") | ("windows10", "x86") | ("windows7", "x86") => 
            vec!["3.8.10", "3.9.13", "3.10.11"].iter().map(|s| s.to_string()).collect(),
        ("windows7", "x64") => 
            vec!["3.8.10", "3.9.13"].iter().map(|s| s.to_string()).collect(),
        ("macos", "x64") => 
            vec!["3.8.10", "3.9.13", "3.10.11", "3.11.7", "3.12.1"].iter().map(|s| s.to_string()).collect(),
        ("macos", "arm64") => 
            vec!["3.9.13", "3.10.11", "3.11.7", "3.12.1"].iter().map(|s| s.to_string()).collect(),
        ("linux", _) => 
            vec!["3.8.10", "3.9.13", "3.10.11", "3.11.7", "3.12.1"].iter().map(|s| s.to_string()).collect(),
        _ => vec!["3.10.11"].iter().map(|s| s.to_string()).collect(), // 默认回退
    }
}

/// 为虚拟环境创建激活脚本
fn create_activation_scripts(venv_dir: &str) -> Result<()> {
    if cfg!(target_os = "windows") {
        let activate_bat = format!(
            r#"@echo off
call {}\\Scripts\\activate.bat
"#, 
            venv_dir
        );
        
        fs::write("activate.bat", activate_bat)
            .context("无法写入activate.bat文件")?;
    } else {
        let activate_sh = format!(
            r#"#!/bin/sh
source {}/bin/activate
"#, 
            venv_dir
        );
        
        fs::write("activate.sh", activate_sh)
            .context("无法写入activate.sh文件")?;
        
        // 使脚本可执行
        Command::new("chmod")
            .args(["+x", "activate.sh"])
            .status()
            .context("无法使activate.sh可执行")?;
    }
    
    println!("创建了激活脚本");
    
    Ok(())
}

/// 将Python文件复制到导出目录
fn copy_python_files(python_files: &[String], export_path: &Path) -> Result<()> {
    let pb = ProgressBar::new(python_files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));
    
    for file in python_files {
        let source_path = Path::new(file);
        let relative_path = source_path.strip_prefix("./").unwrap_or(source_path);
        let target_path = export_path.join("src").join(relative_path);
        
        // 如果父目录不存在则创建
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("无法创建目录: {:?}", parent))?;
        }
        
        // 复制文件
        fs::copy(source_path, &target_path)
            .context(format!("无法复制文件: {:?}", source_path))?;
            
        pb.inc(1);
    }
    
    pb.finish_with_message("文件复制成功");
    
    // 不再需要复制requirements.txt，因为我们会直接在目标目录生成它
    
    Ok(())
}

/// 为目标操作系统创建设置脚本
fn create_setup_scripts(export_path: &Path, python_version: &str, os_type: &str, arch: &str) -> Result<()> {
    if os_type.starts_with("windows") {
        let setup_bat = format!(
            r#"@echo off
echo 正在安装Python {}...
:: 下载Python安装程序
powershell -Command "Invoke-WebRequest -Uri 'https://www.python.org/ftp/python/{}/python-{}-{}.exe' -OutFile 'python-installer.exe'"

:: 安装Python
echo 正在安装Python...
python-installer.exe /quiet InstallAllUsers=0 PrependPath=1 Include_test=0 Include_pip=1

:: 创建虚拟环境
echo 正在创建虚拟环境...
python -m venv .venv

:: 激活虚拟环境
echo 正在激活虚拟环境...
call .venv\Scripts\activate.bat

:: 安装依赖
echo 正在安装依赖...
pip install -r requirements.txt

echo 设置成功完成！
echo 要激活虚拟环境，请运行: .venv\Scripts\activate.bat
"#, 
            python_version, python_version, python_version, 
            if arch == "x86" { "win32" } else { "amd64" }
        );
        
        fs::write(export_path.join("setup.bat"), setup_bat)
            .context("无法写入setup.bat文件")?;
            
        // 创建activate.bat
        let activate_bat = r#"@echo off
call .venv\Scripts\activate.bat
"#;
        
        fs::write(export_path.join("activate.bat"), activate_bat)
            .context("无法写入activate.bat文件")?;
    } else {
        // 对于Linux/macOS
        let setup_sh = format!(
            r#"#!/bin/bash
echo "正在安装Python {}..."

# 创建虚拟环境
python3 -m venv .venv

# 激活虚拟环境
source .venv/bin/activate

# 安装依赖
pip install -r requirements.txt

echo "设置成功完成！"
echo "要激活虚拟环境，请运行: source .venv/bin/activate"
"#, 
            python_version
        );
        
        fs::write(export_path.join("setup.sh"), setup_sh)
            .context("无法写入setup.sh文件")?;
            
        // 创建activate.sh
        let activate_sh = r#"#!/bin/bash
source .venv/bin/activate
"#;
        
        fs::write(export_path.join("activate.sh"), activate_sh)
            .context("无法写入activate.sh文件")?;
    }
    
    println!("创建了设置脚本");
    
    Ok(())
}

/// 创建README文件
fn create_readme(export_path: &Path, python_version: &str, os_name: &str) -> Result<()> {
    let readme = format!(
        r#"# PyWand导出包

此包包含用于离线开发的Python依赖项。

## 系统要求

- 操作系统: {}
- Python版本: {}

## 设置说明

### Windows

1. 运行`setup.bat`安装Python并设置虚拟环境
2. 设置完成后，运行`activate.bat`激活虚拟环境
3. 使用激活的环境运行Python脚本

### Linux/macOS

1. 确保已安装Python {}
2. 运行`chmod +x setup.sh activate.sh`使脚本可执行
3. 运行`./setup.sh`设置虚拟环境
4. 设置完成后，运行`source activate.sh`激活虚拟环境
5. 使用激活的环境运行Python脚本

## 内容

- `src/` - Python源文件
- `requirements.txt` - Python依赖项
- `setup.bat`/`setup.sh` - 设置脚本
- `activate.bat`/`activate.sh` - 激活脚本

## 故障排除

如果遇到任何问题：
- 确保已安装正确的Python版本
- 检查操作系统是否兼容
- 确保在初始设置期间有互联网访问
"#,
        os_name, python_version, python_version
    );
    
    fs::write(export_path.join("README.md"), readme)
        .context("无法写入README.md文件")?;
        
    println!("创建了README文件");
    
    Ok(())
}

/// 创建tar.gz归档
fn create_archive(source_dir: &Path, output_file: &str) -> Result<()> {
    println!("正在创建归档{}...", output_file);
    
    let tar_gz = fs::File::create(output_file)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    
    // 将目录中的所有文件添加到归档
    for entry in WalkDir::new(source_dir) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let relative_path = path.strip_prefix(source_dir)?;
            tar.append_path_with_name(path, relative_path)?;
        }
    }
    
    tar.finish()?;
    
    println!("归档创建成功");
    
    Ok(())
}

/// 显示使用提示，使用指定的语言
fn show_usage_tips_with_language(language: Language) {
    // 创建一个i18n实例，使用指定的语言
    let i18n = I18n::with_language(language);
    
    println!("\n{}", style(i18n.get("usage_tips")).bold().green());
    println!("1. {} - pywand gen-req", style(i18n.get("scan_create_req")).bold());
    println!("2. {} - pywand local-dev", style(i18n.get("setup_local_dev")).bold());
    println!("3. {} - pywand", style(i18n.get("export_to_other")).bold());
    println!("4. {} - pywand run <脚本>", style(i18n.get("run_python_script")).bold());
    println!("5. {} - pywand uv <命令>", style(i18n.get("execute_uv_command")).bold());
    println!("6. {} - pywand pip <包名...>", style(i18n.get("install_python_packages")).bold());
    println!("7. {} - pywand lang --code <语言代码>", style(i18n.get("set_interface_language")).bold());
    println!("   {}: en, zh, ja, ko, fr, de, ru", style(i18n.get("available_languages")).bold());
}

/// 显示使用提示
fn show_usage_tips() {
    // 使用该函数调用带语言参数的版本
    show_usage_tips_with_language(Language::default());
}

/// 保存语言偏好设置到配置文件
fn save_language_preference(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 确保配置目录存在
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "无法确定配置目录路径".to_string())?
        .join("pywand");
    
    std::fs::create_dir_all(&config_dir)?;
    
    // 保存语言代码到配置文件
    let config_file = config_dir.join("language.txt");
    std::fs::write(config_file, code)?;
    
    Ok(())
}

/// 从配置文件加载语言设置
fn load_language_preference() -> Option<Language> {
    // 尝试读取配置文件
    let config_file = dirs::config_dir()?.join("pywand").join("language.txt");
    let code = std::fs::read_to_string(config_file).ok()?;
    let code = code.trim();
    
    // 将语言代码转换为Language枚举
    match code {
        "en" => Some(Language::English),
        "zh" => Some(Language::Chinese),
        "ja" => Some(Language::Japanese),
        "ko" => Some(Language::Korean),
        "fr" => Some(Language::French),
        "de" => Some(Language::German),
        "ru" => Some(Language::Russian),
        _ => None
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Analyze { path }) => {
            let mut app = PyWand::new();
            let dir = path.as_deref().unwrap_or(".");
            app.find_python_files(dir)?;
            app.extract_dependencies()?;
        },
        Some(Commands::Test { path }) => {
            let mut app = PyWand::new();
            println!("{}", style(app.i18n.get("running_in_test")).bold().yellow());
            
            let using_dir_msg = app.i18n.get_formatted(
                "using_directory", 
                &[path]
            );
            println!("{}", using_dir_msg);
            
            app.find_python_files(path)?;
            app.extract_dependencies()?;
            app.show_main_menu()?;
        },
        Some(Commands::LocalDev { path }) => {
            let mut app = PyWand::new();
            println!("{}", style(app.i18n.get("running_local_dev")).bold().yellow());
            
            let using_dir_msg = app.i18n.get_formatted(
                "using_directory", 
                &[path]
            );
            println!("{}", using_dir_msg);
            
            app.find_python_files(path)?;
            app.extract_dependencies()?;
            app.local_development_flow()?;
        },
        Some(Commands::GenReq { path, output }) => {
            let mut app = PyWand::new();
            println!("{}", style(app.i18n.get("generating_req")).bold().yellow());
            
            // 正确处理占位符
            let path_str = path.as_str();
            let output_str = output.as_str();
            
            // 使用格式化后的字符串
            let scanning_dir_msg = format!("扫描目录: {}", path_str);
            println!("{}", scanning_dir_msg);
            
            let output_dir_msg = format!("输出目录: {}", output_str);
            println!("{}", output_dir_msg);
            
            app.find_python_files(&path)?;
            app.extract_dependencies()?;
            app.generate_requirements_file(&output)?;
            
            println!("{}", style(app.i18n.get("req_generated")).bold().green());
        },
        Some(Commands::Run { script, args }) => {
            let mut app = PyWand::new();
            println!("{}", style(app.i18n.get("running_script")).bold().yellow());
            
            let script_msg = app.i18n.get_formatted(
                "script", 
                &[script]
            );
            println!("{}", script_msg);
            
            // 确保存在虚拟环境
            let venv_dir = ".venv";
            if !Path::new(venv_dir).exists() {
                println!("未检测到虚拟环境，正在创建...");
                let python_version = app.select_python_version()?;
                app.uv_manager.create_venv(venv_dir, &python_version)?;
                
                // 如果当前目录存在requirements.txt，则安装依赖
                if Path::new("requirements.txt").exists() {
                    println!("检测到requirements.txt，正在安装依赖...");
                    app.uv_manager.install_dependencies("requirements.txt", venv_dir)?;
                } else {
                    // 扫描并生成requirements.txt
                    println!("未检测到requirements.txt，正在扫描并生成...");
                    app.find_python_files(".")?;
                    app.extract_dependencies()?;
                    if !app.dependencies.is_empty() {
                        app.generate_requirements_file(".")?;
                        app.uv_manager.install_dependencies("requirements.txt", venv_dir)?;
                    }
                }
            }
            
            // 使用内置的uv运行脚本
            println!("{}", style("正在运行脚本...").bold().green());
            let uv_cmd = match app.get_internal_uv_path() {
                Some(path) => path.to_path_buf(),
                None => PathBuf::from(if cfg!(windows) { "uv.exe" } else { "uv" }),
            };
            
            let status = Command::new(uv_cmd)
                .args(["run", script])
                .args(args)
                .status()
                .context("无法运行脚本")?;
            
            if status.success() {
                println!("{}", style("脚本执行成功!").bold().green());
            } else {
                println!("{}", style("脚本执行失败!").bold().red());
                if let Some(code) = status.code() {
                    println!("退出码: {}", code);
                }
            }
            
            // 显示使用提示
            show_usage_tips_with_language(app.i18n.language);
        },
        Some(Commands::Uv { args }) => {
            println!("{}", style("执行UV命令").bold().yellow());
            
            let mut app = PyWand::new();
            
            // 使用内置的uv执行命令
            let uv_cmd = match app.get_internal_uv_path() {
                Some(path) => path.to_path_buf(),
                None => PathBuf::from(if cfg!(windows) { "uv.exe" } else { "uv" }),
            };
            
            let status = Command::new(uv_cmd)
                .args(args)
                .status()
                .context("无法执行UV命令")?;
            
            if status.success() {
                println!("{}", style("UV命令执行成功!").bold().green());
            } else {
                println!("{}", style("UV命令执行失败!").bold().red());
                if let Some(code) = status.code() {
                    println!("退出码: {}", code);
                }
            }
            
            // 显示使用提示
            show_usage_tips_with_language(app.i18n.language);
        },
        Some(Commands::Pip { packages }) => {
            let mut app = PyWand::new();
            println!("{}", style(app.i18n.get("installing_packages")).bold().yellow());
            
            // 检查并确保虚拟环境存在
            let venv_dir = ".venv";
            if !Path::new(venv_dir).exists() {
                println!("未检测到虚拟环境，正在创建...");
                let python_version = app.select_python_version()?;
                
                let creating_venv_msg = app.i18n.get_formatted(
                    "creating_venv", 
                    &[&python_version]
                );
                println!("\n{}", creating_venv_msg);
                
                // 确保UV可用
                app.uv_manager.ensure_available()?;
                
                // 创建虚拟环境
                app.uv_manager.create_venv(venv_dir, &python_version)?;
                
                // 创建激活脚本
                create_activation_scripts(venv_dir)?;
                
                println!("{}", style(app.i18n.get("created_activation_scripts")).bold().green());
            }
            
            // 使用内置的uv安装包
            println!("{}", style(app.i18n.get("installing_dependencies")).bold().green());
            
            // 使用venv中的pip来安装包
            let pip_path = if cfg!(windows) {
                format!("{}/Scripts/pip.exe", venv_dir)
            } else {
                format!("{}/bin/pip", venv_dir)
            };
            
            // 使用venv的pip安装包
            let mut command = Command::new(&pip_path);
            command.arg("install");
            command.args(packages);
            
            let status = command
                .status()
                .context(format!("无法安装包，pip路径：{}", pip_path))?;
            
            if status.success() {
                println!("{}", style(app.i18n.get("packages_installed")).bold().green());
            } else {
                println!("{}", style(app.i18n.get("packages_install_failed")).bold().red());
                if let Some(code) = status.code() {
                    println!("退出码: {}", code);
                }
            }
            
            // 显示使用提示
            show_usage_tips_with_language(app.i18n.language);
        },
        Some(Commands::Lang { code }) => {
            let app = PyWand::new();
            
            let language = match code.as_str() {
                "en" => Language::English,
                "zh" => Language::Chinese,
                "ja" => Language::Japanese,
                "ko" => Language::Korean,
                "fr" => Language::French,
                "de" => Language::German,
                "ru" => Language::Russian,
                _ => {
                    let unsupported_msg = app.i18n.get_formatted(
                        "unsupported_language",
                        &[code]
                    );
                    println!("{}", unsupported_msg);
                    Language::default()
                }
            };
            
            // 由于app不能修改，我们创建一个新的i18n实例
            let i18n = I18n::with_language(language);
            println!("{}", style(i18n.get("language_changed")).bold().green());
            
            // 保存语言设置到配置文件
            if let Err(e) = save_language_preference(&code) {
                println!("Warning: Could not save language preference: {}", e);
            }
            
            // 显示使用提示，使用指定的语言
            show_usage_tips_with_language(language);
        },
        None => {
            let mut app = PyWand::new();
            println!("{}", style(app.i18n.get("no_command")).bold().yellow());
            println!("{}", app.i18n.get("scanning_current"));
            
            // 默认在当前目录查找Python文件
            app.find_python_files(".")?;
            app.extract_dependencies()?;
            app.show_main_menu()?;
        }
    }
    
    Ok(())
}
