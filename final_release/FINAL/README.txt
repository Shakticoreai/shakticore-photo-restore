SHAKTICORE PHOTO RESTORE v1.0 - INSTALLATION GUIDE
==================================================

⚠️ IMPORTANT: MSI REQUIRES EXTRA STEP FOR MODELS


OPTION 1: PORTABLE VERSION (EASIEST - RECOMMENDED) ✅
------------------------------------------------------
Folder: portable\

NO SETUP NEEDED - JUST RUN!
✅ Extract "portable" folder anywhere
✅ Run shakticore-photo-restore.exe
✅ Done! Models already included.

This is the EASIEST and RECOMMENDED way to use ShaktiCore.


OPTION 2: MSI INSTALLER (Requires model installation)
------------------------------------------------------
File: ShaktiCore-PhotoRestore-v1.0-FIXED.msi

STEP 1: Install MSI
1. Run ShaktiCore-PhotoRestore-v1.0-FIXED.msi
2. Follow installer steps
3. Click Finish

STEP 2: Install Models (REQUIRED!)
Choose ONE method:

METHOD A: Automatic (Easiest)
1. Right-click install-models-for-msi.ps1
2. Select "Run with PowerShell"
3. Done! Models installed automatically.

METHOD B: Manual
1. Copy the "models" folder from this ZIP
2. Paste to: C:\Program Files\shakticore-photo-restore\
3. Final structure:
   C:\Program Files\shakticore-photo-restore\
   ├── shakticore-photo-restore.exe
   └── models\
       ├── fast.onnx
       └── quality.onnx

STEP 3: Launch
- Find "shakticore-photo-restore" in Start Menu
- Or run from: C:\Program Files\shakticore-photo-restore\


WHY MODELS AREN'T AUTO-INSTALLED WITH MSI?
-------------------------------------------
Tauri's MSI bundler has limitations with external resources.
We provide the post-install script to make it easy.

For hassle-free experience, use the PORTABLE version!


FEATURES
--------
✅ AI Photo Enhancement (2x upscaling)
✅ Fast mode: 1-2 seconds per image
✅ Quality mode: 7-8 seconds per image
✅ 100% offline - no internet needed
✅ Drag & drop interface
✅ Quality protection (prevents over-processing)
✅ Smart file filtering


SYSTEM REQUIREMENTS
-------------------
• Windows 10 or 11 (64-bit)
• 4GB RAM minimum
• 500MB free disk space


TROUBLESHOOTING
---------------
Q: "Model file not found" error after MSI install?
A: Run install-models-for-msi.ps1 script
   OR manually copy models folder to C:\Program Files\shakticore-photo-restore\

Q: Script won't run?
A: Right-click → "Run with PowerShell"
   If blocked, run: Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass

Q: Still not working?
A: Use the PORTABLE version instead! It always works.


SUPPORT
-------
• GitHub: github.com/vipulchaudhary/shakticore
• Email: vipul@shakticore.com


© 2026 ShaktiCore
