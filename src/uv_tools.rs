use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;

use anyhow::{Context, Result, bail};
use include_dir::{include_dir, Dir};
use console::style;
use rand::Rng;
use dirs::home_dir;

// 嵌入UV二进制文件
// 注意：这里仅是结构，实际的二进制文件需要手动下载并放入resources目录
static UV_RESOURCES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources/uv");

/// UV管理工具
pub struct UvManager {
    bin_path: Option<PathBuf>,
    os_type: String,
    arch: String,
}

impl UvManager {
    /// 创建新的UV管理器
    pub fn new() -> Self {
        UvManager {
            bin_path: None,
            os_type: determine_os_type(),
            arch: determine_os_arch(),
        }
    }

    /// 确保UV可用，如果不可用则解压内置版本
    pub fn ensure_available(&mut self) -> Result<PathBuf> {
        // 首先检查系统中是否已经安装UV
        if let Ok(path) = self.find_system_uv() {
            println!("找到系统安装的UV: {}", path.display());
            self.bin_path = Some(path.clone());
            return Ok(path);
        }

        // 如果系统中没有UV，尝试使用内置的UV
        println!("{}", style("系统中未找到UV，使用内置版本...").yellow());
        
        let bin_path = self.extract_embedded_uv()?;
        self.bin_path = Some(bin_path.clone());
        
        Ok(bin_path)
    }

    /// 在系统PATH中查找UV
    fn find_system_uv(&self) -> Result<PathBuf> {
        let uv_command = if cfg!(target_os = "windows") { "uv.exe" } else { "uv" };
        
        // 使用which命令查找uv
        let result = if cfg!(target_os = "windows") {
            Command::new("where")
                .arg(uv_command)
                .output()
        } else {
            Command::new("which")
                .arg(uv_command)
                .output()
        };

        match result {
            Ok(output) if output.status.success() => {
                if let Ok(path_str) = String::from_utf8(output.stdout) {
                    let path_str = path_str.trim();
                    return Ok(PathBuf::from(path_str));
                }
            }
            _ => {}
        }

        bail!("系统中未找到UV")
    }

    /// 解压内置的UV二进制文件
    fn extract_embedded_uv(&self) -> Result<PathBuf> {
        // 确定对应操作系统和架构的UV二进制文件路径
        let uv_file_name = if cfg!(target_os = "windows") {
            "uv.exe"
        } else {
            "uv"
        };

        // 构建资源路径
        let resource_path = format!("{}-{}/{}", self.os_type, self.arch, uv_file_name);
        
        let uv_data = match UV_RESOURCES.get_file(&resource_path) {
            Some(file) => file.contents(),
            None => {
                // 如果找不到内置的二进制文件，尝试从网络下载
                println!("内置UV二进制文件不可用，尝试从网络下载...");
                return self.download_uv();
            }
        };

        // 创建临时目录来存放UV二进制文件
        let app_dir = get_app_dir()?;
        let bin_dir = app_dir.join("bin");
        fs::create_dir_all(&bin_dir)
            .context("无法创建应用程序目录")?;

        // 写入二进制文件
        let uv_path = bin_dir.join(uv_file_name);
        let mut file = File::create(&uv_path)
            .context("无法创建UV执行文件")?;
        
        file.write_all(uv_data)
            .context("无法写入UV二进制数据")?;
        
        // 在Unix系统上设置可执行权限
        if !cfg!(target_os = "windows") {
            Command::new("chmod")
                .args(["+x", uv_path.to_str().unwrap()])
                .status()
                .context("无法设置UV执行权限")?;
        }

        println!("已解压UV到: {}", uv_path.display());
        Ok(uv_path)
    }

    /// 从网络下载UV
    fn download_uv(&self) -> Result<PathBuf> {
        println!("正在从网络下载UV...");
        
        // 创建临时目录
        let app_dir = get_app_dir()?;
        let bin_dir = app_dir.join("bin");
        fs::create_dir_all(&bin_dir)
            .context("无法创建应用程序目录")?;
        
        let uv_file_name = if cfg!(target_os = "windows") {
            "uv.exe"
        } else {
            "uv"
        };
        
        let uv_path = bin_dir.join(uv_file_name);
        
        // 下载UV安装脚本并执行
        if cfg!(target_os = "windows") {
            let script_path = app_dir.join("uv-installer.ps1");
            let url = "https://github.com/astral-sh/uv/releases/latest/download/uv-installer.ps1";
            
            // 下载安装脚本
            let mut response = reqwest::blocking::get(url)
                .context("无法下载UV安装程序")?;
            
            let mut file = File::create(&script_path)
                .context("无法创建安装脚本文件")?;
            
            std::io::copy(&mut response, &mut file)
                .context("无法保存安装脚本")?;
            
            // 执行安装脚本，将UV安装到我们的应用目录
            Command::new("powershell")
                .args(["-ExecutionPolicy", "Bypass", "-File", script_path.to_str().unwrap()])
                .env("UV_INSTALL_PATH", bin_dir.to_str().unwrap())
                .status()
                .context("无法执行UV安装脚本")?;
        } else {
            let script_path = app_dir.join("uv-installer.sh");
            let url = "https://astral.sh/uv/install.sh";
            
            // 下载安装脚本
            let mut response = reqwest::blocking::get(url)
                .context("无法下载UV安装程序")?;
            
            let mut file = File::create(&script_path)
                .context("无法创建安装脚本文件")?;
            
            std::io::copy(&mut response, &mut file)
                .context("无法保存安装脚本")?;
            
            // 设置执行权限
            Command::new("chmod")
                .args(["+x", script_path.to_str().unwrap()])
                .status()
                .context("无法设置安装脚本执行权限")?;
            
            // 执行安装脚本，将UV安装到我们的应用目录
            Command::new("sh")
                .arg(script_path.to_str().unwrap())
                .env("UV_INSTALL_PATH", bin_dir.to_str().unwrap())
                .status()
                .context("无法执行UV安装脚本")?;
        }
        
        // 检查文件是否存在
        if !uv_path.exists() {
            bail!("UV安装失败，无法找到二进制文件");
        }
        
        println!("已下载UV到: {}", uv_path.display());
        Ok(uv_path)
    }
    
    /// 获取UV路径
    pub fn get_path(&self) -> Option<&PathBuf> {
        self.bin_path.as_ref()
    }
    
    /// 运行UV命令
    pub fn run_command(&self, args: &[&str]) -> Result<()> {
        let uv_path = match self.bin_path.as_ref() {
            Some(path) => path,
            None => bail!("UV未初始化"),
        };
        
        let status = Command::new(uv_path)
            .args(args)
            .status()
            .context("无法执行UV命令")?;
            
        if !status.success() {
            bail!("UV命令执行失败");
        }
        
        Ok(())
    }
    
    /// 创建虚拟环境
    pub fn create_venv(&self, venv_dir: &str, python_version: &str) -> Result<()> {
        println!("使用Python {}创建虚拟环境...", python_version);
        
        self.run_command(&["venv", venv_dir, &format!("--python={}", python_version)])
    }
    
    /// 安装依赖
    pub fn install_dependencies(&self, requirements_file: &str, venv_dir: &str) -> Result<()> {
        // 检查requirements文件是否存在
        if !Path::new(requirements_file).exists() {
            println!("未找到{}文件，跳过依赖安装", requirements_file);
            return Ok(());
        }
        
        // 获取虚拟环境中Python的路径
        let python_path = if cfg!(target_os = "windows") {
            format!("{}\\Scripts\\python.exe", venv_dir)
        } else {
            format!("{}/bin/python", venv_dir)
        };
        
        println!("安装依赖...");
        self.run_command(&["pip", "install", "-r", requirements_file, "--python", &python_path])
    }
}

/// 获取应用程序数据目录
fn get_app_dir() -> Result<PathBuf> {
    let app_dir = if let Some(home) = home_dir() {
        home.join(".pywand")
    } else {
        // 如果找不到home目录，使用临时目录
        let mut rng = rand::thread_rng();
        let random_id: u32 = rng.gen();
        env::temp_dir().join(format!("pywand-{}", random_id))
    };
    
    // 确保目录存在
    fs::create_dir_all(&app_dir)
        .context("无法创建应用程序数据目录")?;
    
    Ok(app_dir)
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