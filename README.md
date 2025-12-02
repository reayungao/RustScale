# RustScale Desktop

**The definitive AI-powered image upscaler for 2025.**

RustScale is a modern desktop application that uses state-of-the-art AI models to upscale and enhance your images with unprecedented quality and performance.

## Why RustScale?

Most desktop upscalers are built on outdated technology—heavy Electron frameworks consuming hundreds of megabytes of RAM, clunky CLI wrappers, and rigid model support. RustScale breaks this mold:

- **10x More Efficient**: Built with Tauri v2 and Rust, using only ~30MB of RAM (vs ~400MB for Electron-based apps)
- **Industry-Standard AI**: Uses ONNX Runtime with support for models from HuggingFace, PyTorch, and research papers
- **Smart Hardware Support**: Automatically detects and optimizes for your GPU, NPU, or CPU
- **Seamless Results**: Advanced mirror-padding tiling eliminates visible grid lines and artifacts
- **Just Works**: Drag and drop ONNX models—no compilation or conversion needed

## Key Features

- **Universal Model Support**: Works with any ONNX super-resolution model
- **Intelligent Batching**: Automatically adjusts performance based on your hardware
- **Face Recovery**: Optional facial enhancement using CodeFormer
- **Real-Time Preview**: Compare before/after with pan and zoom
- **Batch Processing**: Queue multiple images or entire folders
- **Zero Configuration**: Smart defaults with advanced controls when you need them

## Built with AI Assistance

**Full transparency**: This project was built using AI coding agents, specifically Google Antigravity, Gemini 3, and Claude Sonnet 4.5.

I don't have extensive coding experience, but I had a clear vision for an upscaler that respects both user hardware and modern AI capabilities. Agentic AI coding tools helped me turn that vision into reality—an app I genuinely want to use.

**If you have reservations about AI-assisted development, that's completely valid, and you're free not to use this app.** I believe in transparency about the tools used in creation. What matters most is whether the end result serves users well.

That said, I'm always open to feedback, suggestions, and contributions that can make RustScale even better.

## How It Works

RustScale uses a "Native Integration Architecture" instead of spawning external processes. The AI runtime runs directly in the application's memory space, enabling:

- Zero-latency communication between UI and AI
- Direct VRAM management
- Real-time progress tracking
- Instant cancellation

The engine adapts to each model's requirements rather than forcing models to conform to rigid specifications—supporting FP16/FP32 precision, various input shapes, and automatic scale detection.

## Included Models

- **SPAN**: Lightning-fast upscaling for low-resolution images
- **RealPLKSR**: Balanced quality and performance
- **SAFMNv3**: Next-gen Efficienct Model
- **Real-ESRGANv3**: Legacy Univesal Model

## Requirements

- **Windows**: Windows 10/11 with DirectML support
- **Linux**: Ubuntu 20.04+ or equivalent

## Installation

Download the latest release for your platform from the [Releases](../../releases) page:

- Windows: `.exe` installer
- Linux: `.AppImage`

## Usage

1. Launch RustScale
2. Select your scenario (Fast, Balanced, Quality, or Digital Art)
3. Drag and drop images or folders
4. Adjust settings if needed (repair strength, face recovery)
5. Click upscale and watch the magic happen

## Performance Tips

- **Laptop users**: The app automatically enables Eco Mode on battery power
- **Limited VRAM**: The engine adjusts tile sizes based on available memory
- **Batch processing**: Higher batch sizes improve throughput on powerful GPUs

## Contributing

Contributions are welcome! Whether it's bug reports, feature requests, or code improvements, I'd love to hear from you.

## Acknowledgments

- Built with [Tauri v2](https://tauri.app/)
- Powered by [ONNX Runtime](https://onnxruntime.ai/)
- UI framework: [Svelte 5](https://svelte.dev/)
- AI coding assistance: Google Antigravity, Gemini 3, Claude Sonnet 4.5
