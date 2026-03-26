# SpeakEasy

一个基于 Rust 开发的 TTS（文本转语音）桌面应用程序，使用小米 TTS API 实现高质量的语音合成。

## 功能特性

- **文本转语音**：支持单文本和批量模式转换
- **多音色选择**：提供 mimo_default、default_zh、default_en 等预置音色
- **风格控制**：支持通过 `<style>` 标签控制语音风格（情绪、方言、角色扮演等）
- **细粒度控制**：通过音频标签精准调节语气、情绪、语速等
- **内置播放**：生成后可直接播放试听
- **配置管理**：首次运行引导配置，支持界面修改
- **时间戳命名**：自动以时间戳命名生成的文件

## 语音风格示例

### 整体风格控制
```
<style>开心</style>明天就是周五了，真开心！
<style>东北话</style>哎呀妈呀，这天儿也忒冷了吧！
<style>粤语</style>呢个真係好正啊！
```

### 细粒度控制
```
（紧张，深呼吸）呼……冷静，冷静。不就是一个面试吗……
（极其疲惫，有气无力）师傅……到地方了叫我一声……
```

## 技术栈

- **Rust** - 系统编程语言
- **egui** - 即时模式 GUI 框架
- **小米 TTS API** - 语音合成服务
- **rodio** - 音频播放库

## 安装使用

### 方式一：下载安装包

前往 [Releases](https://github.com/qq296565302/tts-rust/releases) 页面下载最新安装包。

### 方式二：从源码构建

#### 环境要求

- Rust 1.70+
- Windows / macOS / Linux

#### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/qq296565302/tts-rust.git
cd tts-rust

# 构建
cargo build --release

# 运行
cargo run --release
```

## 配置说明

首次运行时会弹出配置向导，需要填写：

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| API 密钥 | 小米 TTS API 密钥 | - |
| API 端点 | API 服务地址 | https://api.xiaomimimo.com/v1 |
| 模型名称 | TTS 模型 | mimo-v2-tts |
| 输出目录 | 音频文件保存位置 | 文档/tts_output |

配置文件存储位置：
- Windows: `%APPDATA%\tts_tool\config.json`
- macOS: `~/Library/Application Support/tts_tool/config.json`
- Linux: `~/.config/tts_tool/config.json`

## 项目仓库

- GitHub: https://github.com/qq296565302/tts-rust
- Gitee: https://gitee.com/zach2019/tts-rust

## 许可证

本项目采用 [CC BY-NC 4.0](LICENSE) 协议开源。

- ✅ 个人学习、研究、非商业用途：免费使用
- ❌ 商业用途：需联系作者获取授权

### 商业授权

如需商业使用，请通过 GitHub Issues 联系作者获取授权。

## 作者

赵世俊
