# SpeakEasy

一个基于 Rust 开发的 TTS（文本转语音）桌面应用程序。

## 功能特性

- 文本转语音：支持单文本和批量模式转换
- 多音色选择：提供多种预置音色
- 语速调节：可自定义语音播放速度
- 内置播放：生成后可直接播放试听
- 配置管理：支持界面和配置文件双重管理
- 时间戳命名：自动以时间戳命名生成的文件

## 技术栈

- Rust + egui (GUI框架)
- 小米 TTS API
- rodio (音频播放)

## 快速开始

### 环境要求

- Rust 1.70+
- Windows / macOS / Linux

### 配置

1. 复制配置文件模板：
```bash
cp .env.example .env
```

2. 编辑 `.env` 文件，填入你的 API 配置：
```
TTS_API_KEY=your_api_key
TTS_MODEL=mimo-v2-tts
TTS_BASE_URL=https://api.xiaomimimo.com/v1
```

### 运行

```bash
cargo run
```

### 构建

```bash
cargo build --release
```

## 许可证

本项目采用 [CC BY-NC 4.0](LICENSE) 协议开源。

- 个人学习、研究、非商业用途：免费使用
- 商业用途：需联系作者获取授权

### 商业授权

如需商业使用，请通过 GitHub Issues 联系作者获取授权。
