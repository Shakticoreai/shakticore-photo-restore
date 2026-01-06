# 🖼️ ShaktiCore Photo Restore

**Offline AI Photo Enhancement for Windows** - Enhance your photos with deep learning, 100% offline.

[![Download](https://img.shields.io/badge/Download-v1.0.0-blue)](https://github.com/vipulchauhan/shakticore-photo-restore/releases/latest)
[![Windows](https://img.shields.io/badge/Platform-Windows%2010%2F11-0078D6)](https://github.com/vipulchauhan/shakticore-photo-restore)
[![License](https://img.shields.io/badge/License-Free%20for%20Personal%20Use-green)](LICENSE)

---

## ✨ Features

- 🚀 **2x AI Upscaling** - Deep learning-powered image enhancement
- ⚡ **Fast Mode** - Process images in 1-2 seconds
- ✨ **Quality Mode** - Best results in 7-8 seconds per image
- 💾 **100% Offline** - No internet required, complete privacy
- 🎯 **Smart Protection** - Prevents over-processing automatically
- 📂 **Open Output Folder** - Quick access to enhanced images
- 📊 **Processing Statistics** - See average time and results
- 🎉 **Completion Popup** - Know when processing is done

---

## 📥 Download

**[Download ShaktiCore Photo Restore v1.0.0](https://github.com/vipulchauhan/shakticore-photo-restore/releases/download/v1.0.0/FINAL.zip)** (110 MB)

Choose your installation method:
- **Portable** - No installation, run immediately ✅ Recommended
- **MSI Installer** - Traditional Windows installation

---

## 🚀 Quick Start

### Portable Version (Easiest)
1. Extract `FINAL.zip`
2. Go to `portable/` folder
3. Run `shakticore-photo-restore.exe`
4. Click to select folder with images
5. Choose Fast or Quality mode
6. Wait for processing
7. Enhanced images saved with `_enhanced` suffix

### MSI Installer
1. Extract `FINAL.zip`
2. Run `ShaktiCore-PhotoRestore-v1.0-COMPLETE.msi`
3. Copy `models/` folder to `C:\Program Files\shakticore-photo-restore\`
4. Launch from Start Menu

---

## 💻 System Requirements

- **OS**: Windows 10 or 11 (64-bit)
- **RAM**: 4GB minimum (8GB recommended)
- **Storage**: 500MB free disk space
- **CPU**: Any modern processor (no GPU required)

---

## 📸 Screenshots

![Main Interface](screenshots/main-interface.png)
*Clean, modern interface with Fast and Quality modes*

![Processing Complete](screenshots/completion-popup.png)
*Detailed statistics and quick folder access*

---

## 🎯 How It Works

ShaktiCore uses the FSRCNN (Fast Super-Resolution Convolutional Neural Network) model running on ONNX Runtime for efficient offline processing.

**Processing Modes:**
- **Fast Mode**: Optimized for speed (~1-2s per image)
- **Quality Mode**: Best results (~7-8s per image)

**Smart Features:**
- Skips files larger than 10MB (likely already enhanced)
- Detects multiple enhancements to prevent quality degradation
- Maintains original files (creates new `_enhanced` versions)

---

## 🛡️ Privacy

- ✅ **100% Offline** - No data sent to cloud
- ✅ **No Telemetry** - No tracking or analytics
- ✅ **Local Processing** - Everything stays on your PC
- ✅ **Open Source** - Code available for review

---

## 🔧 Tech Stack

- **Frontend**: Tauri, HTML/CSS/JavaScript
- **Backend**: Rust
- **AI Runtime**: ONNX Runtime
- **Model**: FSRCNN (Fast Super-Resolution CNN)

---

## 📝 License

Free for personal use. For commercial use, please contact vipul@shakticore.com

---

## 🤝 Contributing

Contributions welcome! Please open an issue or PR.

---

## 🐛 Known Issues

None! This is a stable v1.0 release.

---

## 📧 Support

- **Issues**: [GitHub Issues](https://github.com/vipulchauhan/shakticore-photo-restore/issues)
- **Email**: vipul@shakticore.com

---

## 🙏 Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Desktop app framework
- [ONNX Runtime](https://onnxruntime.ai/) - AI inference
- [FSRCNN](https://arxiv.org/abs/1608.00367) - Super-resolution model

---

## 📊 Roadmap (v1.1)

- [ ] GPU acceleration
- [ ] More AI models
- [ ] Batch processing improvements
- [ ] Custom output folder
- [ ] Before/after preview

---

**Made by Vipul Chauhan** | [Website](https://shakticore.com) | [Twitter](https://twitter.com/vipulchauhan)

---

⭐ **Star this repo if you find it useful!**
