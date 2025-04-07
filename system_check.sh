#!/bin/bash
# PyWand系统状态检查脚本
# 用于检查系统中Python和UV的可用性

echo "========================================"
echo "        PyWand 系统状态检查工具        "
echo "========================================"
echo

# 检查操作系统
echo "操作系统信息:"
uname -a
echo

# 检查是否安装了python
echo "Python版本检查:"
if command -v python &> /dev/null; then
    python --version
    echo "Python路径: $(which python)"
else
    echo "未找到python命令"
fi

if command -v python3 &> /dev/null; then
    python3 --version
    echo "Python3路径: $(which python3)"
else
    echo "未找到python3命令"
fi
echo

# 检查是否安装了pip
echo "Pip版本检查:"
if command -v pip &> /dev/null; then
    pip --version
else
    echo "未找到pip命令"
fi

if command -v pip3 &> /dev/null; then
    pip3 --version
else
    echo "未找到pip3命令"
fi
echo

# 检查是否安装了UV
echo "UV版本检查:"
if command -v uv &> /dev/null; then
    uv --version
    echo "UV路径: $(which uv)"
else
    echo "未找到uv命令 (PyWand将使用内置版本)"
fi
echo

# 检查PyWand软件包是否安装
echo "PyWand安装检查:"
if command -v pywand &> /dev/null; then
    echo "PyWand已安装: $(which pywand)"
else
    echo "未找到pywand命令 (可以通过运行./install.sh进行安装)"
fi
echo

# 检查虚拟环境
echo "虚拟环境检查:"
if [ -d ".venv" ]; then
    echo "当前目录中存在.venv虚拟环境"
    if [ -f ".venv/bin/python" ]; then
        echo "虚拟环境Python版本: $(.venv/bin/python --version 2>&1)"
    fi
else
    echo "当前目录中不存在.venv虚拟环境"
fi
echo

echo "系统检查完成!"
echo "如果遇到任何问题，请参考README.md文件或提交Issue到GitHub仓库。"
echo 