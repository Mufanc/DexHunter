# DexHunter

* 简易脱壳小工具，参考葫芦娃大佬的 [`frida-dexdump`](https://github.com/hluwa/frida-dexdump)，但是由 Rust 编写

## 优势

* 轻量、便捷：只有单一可执行文件，体积不过几 MB

* 非侵入性：无需附加到目标进程，有效规避了某些壳父子进程相互 ptrace 的问题

## 使用

### 下载预编译版本
[![Build](https://img.shields.io/github/actions/workflow/status/Mufanc/DexHunter/build.yml?branch=master&label=build)](https://github.com/Mufanc/DexHunter/actions)

* 部署至 Android 设备

```shell
adb push hunter /data/local/tmp
adb shell chmod +x /data/local/tmp/hunter
```

* 运行

```shell
adb shell
su 
cd /data/local/tmp
./hunter --pid '$(pidof com.example.app)' -o <output_dir>
```

### 手动编译

* 克隆项目

* 配置环境变量

```shell
# replace with your ndk path
export ANDROID_NDK_ROOT="/home/mufanc/Android/Sdk/ndk/25.1.8937393"
```

* 编译 & 运行

```shell
# 基本使用
cargo make run --pid '$(pidof com.example.app)' -o <output_dir>

# 「仅扫描」模式，不会提取 dex 文件
cargo make run --pid '$(pidof com.example.app)' 

# 自动获取顶层 activity 的 pid (依赖 dumpsys)
cargo make run -o <output_dir> 
```

* 暂**不支持**对抹头 dex 文件的搜索
