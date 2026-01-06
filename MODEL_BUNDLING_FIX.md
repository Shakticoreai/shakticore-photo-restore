# Model Bundling Fix - IMPORTANT

## 🔧 **Issue Fixed**

**Problem**: App couldn't find models when run offline/installed  
**Root Cause**: Tauri resource bundling wasn't working correctly  
**Solution**: Models must be placed next to the EXE

## 📦 **Distribution Requirements**

When distributing the app, you MUST include the `models` folder:

```
shakticore-photo-restore.exe
models/
  ├── fast.onnx (5.5 MB)
  └── quality.onnx (5.5 MB)
```

## 🚀 **How to Package for Distribution**

### **Step 1: Copy Models to Release**

```powershell
cd D:\My_Project\Shakticore\shakticore-photo-restore\src-tauri
xcopy /E /I /Y models target\release\models
```

### **Step 2: Create Distribution ZIP**

```powershell
# Create dist folder with EXE and models
New-Item -ItemType Directory -Force -Path dist
Copy-Item target\release\shakticore-photo-restore.exe dist\
Copy-Item -Recurse target\release\models dist\

# Create ZIP
Compress-Archive -Path dist\* -DestinationPath ShaktiCore-v1.0-Portable.zip -Force
```

### **Step 3: For MSI Installer**

The MSI installer needs to be updated to include the models folder. Update `tauri.conf.json`:

```json
"bundle": {
  "resources": [
    "models/*"
  ],
  "externalBin": []
}
```

Then rebuild:
```powershell
npm run tauri build
```

## ✅ **Testing**

Before distributing, test that models are found:

1. Copy `target\release\shakticore-photo-restore.exe` to a test folder
2. Copy `target\release\models\` folder to same test folder
3. Run the EXE
4. Try processing images
5. Should work without errors!

## 📝 **For Users**

Include these instructions in README:

```
Installation:
1. Extract the ZIP file
2. Make sure the folder structure is:
   shakticore-photo-restore.exe
   models/
     fast.onnx
     quality.onnx
3. Run shakticore-photo-restore.exe
```

## 🎯 **Summary**

- ✅ App now looks for models next to EXE
- ✅ Works in dev mode (fallback to relative path)
- ✅ Works when installed (exe directory)
- ⚠️ **MUST distribute models folder with EXE**

**The fix is complete - just remember to include the models folder!**
