#!/bin/bash
# PyWand安装脚本

set -e

# 配置变量
INSTALL_DIR="$HOME/.local/bin"
CURRENT_DIR=$(pwd)
BINARY_PATH="$CURRENT_DIR/target/release/pywand"

# 确认要安装的路径
echo "PyWand将被安装到: $INSTALL_DIR"
echo "如果这个路径不在您的PATH环境变量中，您需要手动添加它。"
read -p "是否继续? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "安装已取消"
    exit 1
fi

# 检查二进制文件是否存在
if [ ! -f "$BINARY_PATH" ]; then
    echo "错误: 二进制文件不存在: $BINARY_PATH"
    echo "请先运行 'cargo build --release' 来构建项目"
    exit 1
fi

# 创建安装目录（如果不存在）
mkdir -p "$INSTALL_DIR"

# 复制二进制文件
echo "复制 pywand 到 $INSTALL_DIR..."
cp "$BINARY_PATH" "$INSTALL_DIR/"

# 设置执行权限
chmod +x "$INSTALL_DIR/pywand"

echo "安装成功!"
echo "您现在可以使用 'pywand' 命令来运行程序。"

# 检查安装目录是否在PATH中
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo
    echo "注意: 安装目录似乎不在您的PATH环境变量中。"
    echo "您可能需要将以下行添加到您的~/.bashrc或~/.zshrc文件中:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi 