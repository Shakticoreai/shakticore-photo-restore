# ShaktiCore Photo Restore v1.0

## 🎯 **What is this?**

Free offline AI-powered photo upscaler for Windows. Enhance old family photos, restore quality, and upscale images using advanced AI models.

## ✨ **Features**

- **AI Upscaling**: 2x resolution enhancement using EDSR model
- **Batch Processing**: Process entire folders at once
- **GPU Acceleration**: Uses DirectML for faster processing
- **100% Offline**: No internet required, complete privacy
- **Simple Interface**: Beautiful, easy-to-use design

## 💻 **System Requirements**

- **OS**: Windows 10 or Windows 11 (64-bit)
- **RAM**: 4 GB minimum, 8 GB recommended
- **Storage**: 50 MB for app + space for processed images
- **GPU**: Optional (DirectML-compatible GPU for faster processing)

## 📥 **Installation**

### **Option 1: MSI Installer (Recommended)**

1. Download `shakticore-photo-restore_0.1.0_x64_en-US.msi`
2. Double-click to install
3. Follow the installation wizard
4. Launch from Start Menu

### **Option 2: Portable EXE**

1. Download `shakticore-photo-restore.exe`
2. Double-click to run
3. No installation needed

## 🔒 **Security Warning**

Windows may show: *"Windows protected your PC"*

**This is normal** - the app is not code-signed yet.

**To run**:
1. Click **"More info"**
2. Click **"Run anyway"**

The app is safe and open-source.

## 🚀 **How to Use**

1. **Launch** the app
2. **Select mode**: Fast (⚡) or Quality (✨)
3. **Click** "Select Folder & Process"
4. **Choose** a folder with images
5. **Wait** for processing
6. **Find** enhanced images with `_enhanced` suffix

## 📊 **Supported Formats**

- **Input**: JPG, JPEG, PNG, BMP
- **Output**: Same format as input

## ⏱️ **Performance**

- **Speed**: ~12 seconds per image (GPU)
- **Quality**: 2x upscaling with AI enhancement
- **File Size**: Output files are 10-50x larger (more detail)

## 🐛 **Known Limitations**

- Windows 10/11 only (no Windows 7 support)
- 2x upscaling only (4x coming in v2.0)
- JPEG/PNG output only
- No before/after preview (coming in v1.1)

## 🆘 **Troubleshooting**

### **App won't start**
- Make sure you're on Windows 10/11
- Try running as administrator
- Check antivirus isn't blocking it

### **Processing takes too long**
- First image always takes longer (model loading)
- GPU acceleration requires DirectML-compatible GPU
- Large images (>4K) take longer

### **Files are too large**
- This is normal - upscaled images have more detail
- v1.1 will add quality controls to reduce file size

## 📝 **Version History**

### **v1.0.0** (January 1, 2026)
- Initial release
- EDSR x2 upscaling model
- Batch processing
- GPU acceleration
- Processing time display

## 🔮 **Roadmap**

### **v1.1** (Coming Soon)
- Output quality controls (JPEG 85/90/95)
- Auto-compression for large files
- File size statistics
- Before/after preview

### **v2.0** (Future)
- 4x upscaling (RealESRGAN)
- Face restoration
- Drag-and-drop support
- Multiple output formats

## 📧 **Support**

- **Issues**: Report bugs on GitHub
- **Questions**: Reddit community
- **Updates**: Follow for new releases

## 📄 **License**

Free for personal use. See LICENSE file for details.

## 🙏 **Credits**

- **AI Model**: EDSR (Enhanced Deep Residual Networks)
- **Framework**: Tauri + Rust + ONNX Runtime
- **Developer**: Vipul Chaudhary

---

**Made with ❤️ for preserving memories**

*Restore your old photos, one pixel at a time.*
