# UV二进制文件

本目录用于存放UV工具的二进制文件，以便在PyWand应用程序中内置使用。

## 目录结构

- `macos-arm64/` - macOS ARM64架构的UV二进制文件
- `macos-x64/` - macOS x64架构的UV二进制文件
- `linux-x64/` - Linux x64架构的UV二进制文件
- `windows-x64/` - Windows x64架构的UV二进制文件

## 如何获取UV二进制文件

1. 访问UV的官方GitHub仓库发布页面：https://github.com/astral-sh/uv/releases
2. 下载适合您目标平台的二进制文件
3. 将下载的二进制文件放入相应的目录中
4. 确保二进制文件名称为：
   - Windows: `uv.exe`
   - macOS/Linux: `uv`

## 示例

例如，要为macOS ARM64添加UV二进制文件：

1. 下载macOS ARM64版本的UV
2. 将其重命名为`uv`
3. 放入`macos-arm64/`目录中

## 注意

如果没有在这些目录中提供二进制文件，PyWand将尝试从网络下载安装。这只是一个优化，以避免每次都需要下载。 