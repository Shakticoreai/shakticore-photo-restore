# Build Script for ShaktiCore Photo Restore
# Ensures models are properly bundled in MSI

Write-Host "Building ShaktiCore Photo Restore..." -ForegroundColor Cyan
Write-Host ""

# Step 1: Verify models exist
Write-Host "Step 1: Verifying models..." -ForegroundColor Yellow
if (-not (Test-Path "models\fast.onnx")) {
    Write-Host "ERROR: models\fast.onnx not found!" -ForegroundColor Red
    exit 1
}
if (-not (Test-Path "models\quality.onnx")) {
    Write-Host "ERROR: models\quality.onnx not found!" -ForegroundColor Red
    exit 1
}
Write-Host "Models found" -ForegroundColor Green
Write-Host ""

# Step 2: Copy models to src-tauri directory
Write-Host "Step 2: Copying models to src-tauri..." -ForegroundColor Yellow
if (Test-Path "src-tauri\models") {
    Remove-Item -Recurse -Force "src-tauri\models"
}
Copy-Item -Recurse "models" "src-tauri\models"
Write-Host "Models copied to src-tauri\models" -ForegroundColor Green
Write-Host ""

# Step 3: Build the application
Write-Host "Step 3: Building application..." -ForegroundColor Yellow
npm run tauri build

# Step 4: Verify MSI was created
Write-Host ""
Write-Host "Step 4: Verifying build output..." -ForegroundColor Yellow
$msiPath = "src-tauri\target\release\bundle\msi\shakticore-photo-restore_0.1.0_x64_en-US.msi"
if (Test-Path $msiPath) {
    $size = [math]::Round((Get-Item $msiPath).Length / 1MB, 2)
    Write-Host "MSI created successfully!" -ForegroundColor Green
    Write-Host "Location: $msiPath" -ForegroundColor White
    Write-Host "Size: $size MB" -ForegroundColor White
    Write-Host ""
    Write-Host "Build complete!" -ForegroundColor Green
}
else {
    Write-Host "MSI not found!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "To test the MSI:" -ForegroundColor Cyan
Write-Host "1. Install the MSI" -ForegroundColor White
Write-Host "2. Check C:\Program Files\shakticore-photo-restore\models\" -ForegroundColor White
Write-Host "3. Launch from Start Menu" -ForegroundColor White
