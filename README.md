# PyWand

✨ — 你的Python环境魔法师

只需一键，PyWand自动扫描分析项目文件夹中的Python依赖，从零开始创建全新的隔离环境，自动安装所有依赖包。
完成后，PyWand还能将整个项目连同配置好的环境打包输出，开箱即用，轻松迁移与部署。

## 主要功能

- **智能依赖分析**：自动识别项目中使用的全部Python库，无需手动维护requirements.txt
- **全新环境构建**：从头创建隔离的Python虚拟环境，避免依赖冲突
- **自动安装依赖**：根据分析结果自动安装所需库，省时省心
- **一键打包输出**：将项目与环境打包成可直接运行的文件夹，方便分发和部署
- **开箱即用**：无需繁琐配置，轻松实现项目环境的快速复现
- **内置UV工具**：无需单独安装UV工具，避免依赖其他系统工具

## 开始使用

### 安装

1. 克隆仓库
2. 构建项目：
   ```
   cargo build --release
   ```
3. 可执行文件将位于`target/release/pywand`

### 使用方法

#### 基本用法

在项目目录中直接运行PyWand：

```
./pywand
```

这将显示一个菜单，提供以下选项：
- 设置本地开发环境
- 导出项目以进行离线开发

#### 命令行选项

- 分析特定目录中的依赖关系：
  ```
  ./pywand analyze --path /path/to/project
  ```

- 使用测试套件样例运行：
  ```
  ./pywand test
  ```
  默认情况下，这将使用`test-suite`文件夹中的样例文件。

## 测试套件

`test-suite`文件夹包含各种依赖关系的Python示例文件，用于测试PyWand：

- `sample_script.py`：一个简单的脚本，包含requests、numpy、pandas、Flask、matplotlib和SQLAlchemy等依赖
- `utils.py`：一个实用工具模块，包含pydantic、boto3、rich和pyyaml等更多依赖
- `config.yaml`：实用工具模块使用的示例配置文件

这些文件可用于测试PyWand的依赖分析功能。

## 内置UV工具

PyWand内置了UV工具，无需用户单独安装：

- 首次运行时，如果系统中未检测到UV，PyWand会自动下载并使用内置版本
- 支持的平台包括Windows、macOS和Linux（x64和ARM64架构）
- 所有依赖安装操作都使用内置UV完成，避免对系统Python环境的依赖
- 也支持使用已安装的系统UV版本（如果已存在）

要预先下载UV二进制文件并内置到应用中，请参见`resources/uv/README.md`文件中的说明。

## 许可证

有关更多信息，请参阅LICENSE文件。

## 开发者说明

PyWand基于Rust开发，使用UV作为底层Python环境管理工具。要添加新功能或修复问题，请参考源代码中的注释和文档。

PyWand — 让Python环境搭建像施展魔法一样简单高效！
