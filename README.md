<div align="center">

# 🚀 RustScale Desktop

**The definitive AI-powered image upscaler for 2025.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-v2-24C8DB?logo=tauri)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte)](https://svelte.dev/)
[![ONNX](https://img.shields.io/badge/ONNX-Runtime-005CED?logo=onnx)](https://onnxruntime.ai/)

[Features](#-key-features) • [Installation](#-installation) • [Usage](#-usage) • [Contributing](#-contributing)

</div>

---

## 🎯 Why RustScale?

Most desktop upscalers are built on outdated technology—heavy Electron frameworks consuming hundreds of megabytes of RAM, clunky CLI wrappers, and rigid model support. RustScale breaks this mold:

| Legacy Apps | RustScale |
|------------|-----------|
| 🐌 ~400MB RAM (Electron) | ⚡ ~30MB RAM (Tauri v2) |
| 🔧 CLI wrapper processes | 🧠 In-process AI runtime |
| 📦 Proprietary formats | 🌐 Industry-standard ONNX |
| 🎲 Manual configuration | 🤖 Auto-detects hardware |
| 🔲 Visible grid artifacts | ✨ Seamless tiling |

## ✨ Key Features

<table>
<tr>
<td width="50%">

### 🎨 **Universal Model Support**
Drag and drop any ONNX super-resolution model—no compilation needed.

### ⚙️ **Intelligent Hardware Optimization**
Automatically detects and optimizes for GPU, NPU, or CPU.

### 📦 **Batch Processing**
Queue multiple images or entire folders with real-time progress.

</td>
<td width="50%">

### 🔍 **Real-Time Preview**
Compare before/after with pan, zoom, and slider controls.

### 👤 **Face Recovery**
Optional facial enhancement using CodeFormer integration.

### 🎯 **Zero Configuration**
Smart defaults that just work, with advanced controls when needed.

</td>
</tr>
</table>

## 🤖 Built with AI Assistance

<div align="center">

### 🔬 Full Transparency

</div>

This project was built using **AI coding agents**: Google Antigravity, Gemini 3, and Claude Sonnet 4.5.

I don't have extensive coding experience, but I had a clear vision for an upscaler that respects both user hardware and modern AI capabilities. Agentic AI coding tools helped me turn that vision into reality—an app I genuinely want to use.

> **If you have reservations about AI-assisted development, that's completely valid, and you're free not to use this app.** I believe in transparency about the tools used in creation. What matters most is whether the end result serves users well.

**I'm always open to feedback, suggestions, and contributions that can make RustScale even better.** 💡

## 🏗️ How It Works

RustScale uses a **"Native Integration Architecture"** instead of spawning external processes:

```
┌─────────────┐      ┌──────────────┐      ┌─────────────┐
│   Svelte    │ ───> │  Rust Core   │ ───> │ ONNX Runtime│
│   UI Layer  │ IPC  │  + HAL       │ Zero │  (In-Memory)│
└─────────────┘      └──────────────┘ Copy └─────────────┘
                            │
                    ┌───────┴───────┐
                    │ GPU/NPU/CPU   │
                    └───────────────┘
```

**Benefits:**
- ⚡ Zero-latency communication between UI and AI
- 💾 Direct VRAM management
- 📊 Real-time progress tracking
- ⏹️ Instant cancellation

## 📦 Included Models

| Model | Speed | Quality | Best For |
|-------|-------|---------|----------|
| 🏃 **SPAN** | ⚡⚡⚡ | ⭐⭐⭐ | Fast upscaling for low-res images |
| 🚀 **SAFMN-V3** | ⚡⚡⚡ | ⭐⭐⭐ | Next-gen efficient model |
| 🎨 **NomosUni SPAN** | ⚡⚡ | ⭐⭐⭐ | Digital art and illustrations |
| 💎 **Real-PLSKR** | ⚡ | ⭐⭐⭐⭐ | Maximum quality for photos |
| 🎭 **RealPLSKR Anime** | ⚡ | ⭐⭐⭐⭐ | Best quality for anime & digital art |
| 🔧 **Real-ESRGAN-V3** | ⚡⚡ | ⭐⭐ | Legacy model for compatibility |

## 💻 System Requirements

<table>
<tr>
<td align="center">🪟 <b>Windows</b></td>
<td align="center">🍎 <b>macOS</b></td>
<td align="center">🐧 <b>Linux</b></td>
</tr>
<tr>
<td>Windows 10/11<br>DirectML support</td>
<td>macOS 13+<br>Apple Silicon or Intel</td>
<td>Ubuntu 20.04+<br>or equivalent</td>
</tr>
</table>

## 📥 Installation

<div align="center">

### Download the latest release for your platform:

[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](../../releases)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](../../releases)

</div>

**Installation Files:**
- 🪟 Windows: `.exe` installer 
- 🐧 Linux: `.AppImage`

## 🎬 Usage

1. **Launch** RustScale
2. **Select** your scenario (Fast, Balanced, Quality, or Digital Art)
3. **Drag & drop** images or folders
4. **Adjust** settings if needed (repair strength, face recovery)
5. **Click** upscale and watch the magic happen ✨

## 💡 Performance Tips

| Scenario | Tip |
|----------|-----|
| 💻 **Laptop users** | App automatically enables Eco Mode on battery power |
| 🎮 **Limited VRAM** | Engine adjusts tile sizes based on available memory |
| 🚀 **Batch processing** | Higher batch sizes improve throughput on powerful GPUs |

## 🤝 Contributing

Contributions are welcome! Whether it's:

- 🐛 **Bug reports**
- 💡 **Feature requests**
- 🔧 **Code improvements**
- 📖 **Documentation**

**I'd love to hear from you!**

<div align="center">

[![GitHub Issues](https://img.shields.io/github/issues/yourusername/rustscale?style=flat-square)](../../issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/yourusername/rustscale?style=flat-square)](../../pulls)

</div>

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Built with modern, powerful tools:

- 🦀 [Tauri v2](https://tauri.app/) - Lightweight desktop framework
- 🧠 [ONNX Runtime](https://onnxruntime.ai/) - Cross-platform AI inference
- ⚡ [Svelte 5](https://svelte.dev/) - Reactive UI framework
- 🎨 [TailwindCSS v4](https://tailwindcss.com/) - Utility-first styling
- 🤖 AI Coding Agents: Google Antigravity, Gemini 3, Claude Sonnet 4.5

## 📊 Project Stats

<div align="center">

![GitHub Stars](https://img.shields.io/github/stars/yourusername/rustscale?style=social)
![GitHub Forks](https://img.shields.io/github/forks/yourusername/rustscale?style=social)
![GitHub Watchers](https://img.shields.io/github/watchers/yourusername/rustscale?style=social)

</div>

---

<div align="center">

**RustScale**: Bringing desktop AI upscaling into the modern era.

Made with ❤️ and 🤖

[⬆ Back to Top](#-rustscale-desktop)

</div>