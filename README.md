# genact - 无聊活动生成器

假装很忙或假装在等电脑加载！打开几个 `genact` 实例，看着屏幕上的"高能输出"让别人以为你正忙于重要工作。`genact` 有多个场景，每个场景都在假装执行一些激动人心或有用的操作，实际上什么都没发生。

## 安装

你甚至不需要安装！预编译的 Linux、macOS 和 Windows 二进制文件可以在[发布页](https://github.com/ByteTora/genact/releases)找到，开箱即用。

兼容 FreeBSD、Linux、macOS、Windows，以及支持 WebAssembly 的现代浏览器。

**FreeBSD**:

    pkg install genact
    genact

**Linux**: 从[发布页](https://github.com/ByteTora/genact/releases)下载 `genact-linux` 并运行

    chmod +x genact-linux
    ./genact-linux

**macOS**: 从[发布页](https://github.com/ByteTora/genact/releases)下载 `genact-osx` 并运行

    chmod +x genact-osx
    ./genact-osx

也可通过 Homebrew 安装：

    brew install genact

或通过 MacPorts：

    sudo port install genact

**Windows**: 从[发布页](https://github.com/ByteTora/genact/releases)下载 `genact-win.exe` 并双击运行。

也可通过 Scoop 安装：

    scoop install genact

**通过 Cargo 安装**：

    cargo install genact
    genact

**从源码运行**：

    git clone https://github.com/ByteTora/genact.git
    cd genact
    cargo run --release

## 使用说明

查看所有可用选项：

    ./genact -h

### 所有模块

| 模块 | 模拟命令 | 说明 |
|------|----------|------|
| `ansible` | ansible-playbook | 运行 Ansible 剧本 |
| `bootlog` | dmesg | 系统启动日志 |
| `botnet` | - | 僵尸网络活动 |
| `bruteforce` | hydra | SSH 暴力破解 |
| `cargo` | cargo run | Rust 编译 |
| `cc` | gcc | C 代码编译 |
| `composer` | composer install | PHP 依赖安装 |
| `cryptomining` | - | 加密货币挖矿 |
| `docker_build` | docker build | 构建 Docker 镜像 |
| `docker_image_rm` | docker image rm | 删除 Docker 镜像 |
| `docker_pull` | docker pull | 下载 Docker 镜像 |
| `download` | wget | 文件下载 |
| `julia` | julia | Julia 包安装 |
| `kernel_compile` | make | Linux 内核编译 |
| `llm_train` | torchrun | 训练大语言模型 |
| `memdump` | dd | 内存转储 |
| `mkinitcpio` | mkinitcpio | Arch Linux 初始化 |
| `rkhunter` | rkhunter | Rootkit 扫描 |
| `simcity` | - | SimCity 2000 编译 |
| `terraform` | terraform apply | Terraform 部署 |
| `uv` | uv pip install | Python 包安装 |
| `weblog` | - | Web 服务器日志 |
| `wpt` | - | Web 平台测试 |

### 命令行参数

```
Usage: genact [OPTIONS]

Options:
  -l, --list-modules                                   列出所有可用模块
  -m, --modules <MODULES>                              只运行指定模块 [可选值: ansible, bootlog, ...]
  -s, --speed-factor <SPEED_FACTOR>                    全局速度倍率 [默认: 1]
  -i, --instant-print-lines <INSTANT_PRINT_LINES>      前 N 行瞬间打印 [默认: 0]
      --exit-after-time <EXIT_AFTER_TIME>              运行指定时间后退出 (例如: 2h10min)
      --exit-after-modules <EXIT_AFTER_MODULES>        运行指定模块次数后退出
      --print-completions <shell>                      生成 Shell 自动补全脚本
      --print-manpage                                  生成 man 手册页
  -h, --help                                           打印帮助
  -V, --version                                        打印版本
```

### 使用示例

只跑 Docker 下载场景：

    ./genact -m docker_pull

只跑 LLM 训练场景：

    ./genact -m llm_train

组合多个模块：

    ./genact -m llm_train,docker_pull,download,bootlog

2 倍速运行全部模块，30 秒后退出：

    ./genact -s 2 --exit-after-time 30s

### Web 用法

在浏览器中通过 URL 参数指定模块：
https://svenstaro.github.io/genact?module=cc&module=memdump

设置速度：
https://svenstaro.github.io/genact?speed-factor=5

## 构建

确保安装了较新版本的 Rust 和 Cargo：

    git clone https://github.com/ByteTora/genact.git
    cd genact
    cargo run --release

## 许可

MIT License
