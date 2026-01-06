# Create Distribution Package for ShaktiCore Photo Restore

Write-Host "Creating distribution package..." -ForegroundColor Cyan

# Paths
$releaseDir = "src-tauri\target\release"
$bundleDir = "$releaseDir\bundle"
$distDir = "dist"

# Create dist directory
New-Item -ItemType Directory -Force -Path $distDir | Out-Null

# Copy MSI installer
Write-Host "`nCopying MSI installer..." -ForegroundColor Yellow
$msiPath = "$bundleDir\msi\shakticore-photo-restore_0.1.0_x64_en-US.msi"
if (Test-Path $msiPath) {
    Copy-Item $msiPath "$distDir\ShaktiCore-PhotoRestore-v1.0-Setup.msi"
    $msiSize = [math]::Round((Get-Item "$distDir\ShaktiCore-PhotoRestore-v1.0-Setup.msi").Length / 1MB, 2)
    Write-Host "✅ MSI installer copied ($msiSize MB)" -ForegroundColor Green
}
else {
    Write-Host "❌ MSI installer not found!" -ForegroundColor Red
}

# Copy portable EXE
Write-Host "`nCopying portable EXE..." -ForegroundColor Yellow
$exePath = "$releaseDir\shakticore-photo-restore.exe"
if (Test-Path $exePath) {
    Copy-Item $exePath "$distDir\ShaktiCore-PhotoRestore-v1.0-Portable.exe"
    $exeSize = [math]::Round((Get-Item "$distDir\ShaktiCore-PhotoRestore-v1.0-Portable.exe").Length / 1MB, 2)
    Write-Host "✅ Portable EXE copied ($exeSize MB)" -ForegroundColor Green
}
else {
    Write-Host "❌ Portable EXE not found!" -ForegroundColor Red
}

# Copy README
Write-Host "`nCopying README..." -ForegroundColor Yellow
if (Test-Path "README.md") {
    Copy-Item "README.md" "$distDir\README.txt"
    Write-Host "✅ README copied" -ForegroundColor Green
}

# Create release notes
Write-Host "`nCreating release notes..." -ForegroundColor Yellow
$releaseNotes = @"
ShaktiCore Photo Restore v1.0.0
================================

Release Date: January 1, 2026

What's New:
-----------
- Initial public release
- AI-powered 2x photo upscaling
- Batch processing support
- GPU acceleration (DirectML)
- Beautiful, simple interface
- Processing time display
- 100% offline operation

Files in this package:
---------------------
- ShaktiCore-PhotoRestore-v1.0-Setup.msi (22 MB)
  → Professional installer, recommended for most users
  
- ShaktiCore-PhotoRestore-v1.0-Portable.exe (38 MB)
  → Portable version, no installation needed
  
- README.txt
  → Full documentation and usage guide

Installation:
------------
1. Download the MSI installer
2. Double-click to install
3. Click "More info" → "Run anyway" if Windows shows a warning
4. Launch from Start Menu

Quick Start:
-----------
1. Open the app
2. Select Fast or Quality mode
3. Click "Select Folder & Process"
4. Choose a folder with images
5. Wait for processing to complete
6. Find enhanced images with "_enhanced" suffix

System Requirements:
-------------------
- Windows 10 or 11 (64-bit)
- 4 GB RAM minimum
- 50 MB disk space
- GPU optional (faster with DirectML)

Known Issues:
------------
- First image takes longer (model loading)
- Output files are larger (more detail)
- Windows security warning (app not code-signed)

Support:
-------
Report issues or ask questions on Reddit or GitHub

Enjoy restoring your photos!
"@

$releaseNotes | Out-File -FilePath "$distDir\RELEASE_NOTES.txt" -Encoding UTF8
Write-Host "✅ Release notes created" -ForegroundColor Green

# Create ZIP archive
Write-Host "`nCreating ZIP archive..." -ForegroundColor Yellow
$zipPath = "ShaktiCore-PhotoRestore-v1.0-Windows.zip"
if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}

Compress-Archive -Path "$distDir\*" -DestinationPath $zipPath
$zipSize = [math]::Round((Get-Item $zipPath).Length / 1MB, 2)
Write-Host "✅ ZIP archive created ($zipSize MB)" -ForegroundColor Green

# Summary
Write-Host "`n" -NoNewline
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Distribution Package Created!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📦 Package contents:" -ForegroundColor Yellow
Write-Host "  - MSI Installer ($msiSize MB)" -ForegroundColor White
Write-Host "  - Portable EXE ($exeSize MB)" -ForegroundColor White
Write-Host "  - README.txt" -ForegroundColor White
Write-Host "  - RELEASE_NOTES.txt" -ForegroundColor White
Write-Host ""
Write-Host "📁 Location:" -ForegroundColor Yellow
Write-Host "  $((Get-Location).Path)\$zipPath" -ForegroundColor White
Write-Host ""
Write-Host "🚀 Ready to share!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Upload $zipPath to Google Drive/Dropbox" -ForegroundColor White
Write-Host "  2. Share the download link" -ForegroundColor White
Write-Host "  3. Post on Reddit/social media" -ForegroundColor White
Write-Host ""
