#!/bin/bash
# 下载各平台UV二进制文件的脚本

# 确保目录存在
mkdir -p resources/uv/macos-arm64
mkdir -p resources/uv/macos-x64
mkdir -p resources/uv/linux-x64
mkdir -p resources/uv/windows-x64

# 获取最新版本的UV
UV_VERSION=$(curl -s https://api.github.com/repos/astral-sh/uv/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
echo "下载UV版本: $UV_VERSION"

# 下载各平台的UV

# Windows x64 - 使用zip文件
echo "下载Windows x64版本..."
curl -L -o resources/uv/windows-x64/uv.zip "https://github.com/astral-sh/uv/releases/download/$UV_VERSION/uv-x86_64-pc-windows-msvc.zip"
# 解压缩zip文件
unzip -o resources/uv/windows-x64/uv.zip -d resources/uv/windows-x64/
# 清理
rm -f resources/uv/windows-x64/uv.zip

# Linux x64
echo "下载Linux x64版本..."
curl -L -o resources/uv/linux-x64/uv.tar.gz "https://github.com/astral-sh/uv/releases/download/$UV_VERSION/uv-x86_64-unknown-linux-gnu.tar.gz"
# 解压缩tar.gz文件
tar -xzf resources/uv/linux-x64/uv.tar.gz -C resources/uv/linux-x64
# 移动二进制文件并清理
if [ -f resources/uv/linux-x64/uv-x86_64-unknown-linux-gnu/uv ]; then
  mv resources/uv/linux-x64/uv-x86_64-unknown-linux-gnu/uv resources/uv/linux-x64/uv
  rm -rf resources/uv/linux-x64/uv-x86_64-unknown-linux-gnu resources/uv/linux-x64/uv.tar.gz
else 
  echo "Linux二进制文件结构不符合预期，请手动检查"
fi

# macOS x64
echo "下载macOS x64版本..."
curl -L -o resources/uv/macos-x64/uv.tar.gz "https://github.com/astral-sh/uv/releases/download/$UV_VERSION/uv-x86_64-apple-darwin.tar.gz"
# 解压缩tar.gz文件
tar -xzf resources/uv/macos-x64/uv.tar.gz -C resources/uv/macos-x64
# 移动二进制文件并清理
if [ -f resources/uv/macos-x64/uv-x86_64-apple-darwin/uv ]; then
  mv resources/uv/macos-x64/uv-x86_64-apple-darwin/uv resources/uv/macos-x64/uv
  rm -rf resources/uv/macos-x64/uv-x86_64-apple-darwin resources/uv/macos-x64/uv.tar.gz
else 
  echo "macOS x64二进制文件结构不符合预期，请手动检查"
fi

# macOS ARM64
echo "下载macOS ARM64版本..."
curl -L -o resources/uv/macos-arm64/uv.tar.gz "https://github.com/astral-sh/uv/releases/download/$UV_VERSION/uv-aarch64-apple-darwin.tar.gz"
# 解压缩tar.gz文件
tar -xzf resources/uv/macos-arm64/uv.tar.gz -C resources/uv/macos-arm64
# 移动二进制文件并清理
if [ -f resources/uv/macos-arm64/uv-aarch64-apple-darwin/uv ]; then
  mv resources/uv/macos-arm64/uv-aarch64-apple-darwin/uv resources/uv/macos-arm64/uv
  rm -rf resources/uv/macos-arm64/uv-aarch64-apple-darwin resources/uv/macos-arm64/uv.tar.gz
else 
  echo "macOS ARM64二进制文件结构不符合预期，请手动检查"
fi

# 添加执行权限
chmod +x resources/uv/linux-x64/uv
chmod +x resources/uv/macos-x64/uv
chmod +x resources/uv/macos-arm64/uv

echo "下载完成，请检查各平台目录中的UV二进制文件"
ls -lh resources/uv/*/ 