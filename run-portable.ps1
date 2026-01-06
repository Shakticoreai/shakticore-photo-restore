# Run Portable Version - Helper Script

Write-Host "🚀 ShaktiCore Photo Restore - Portable Launcher" -ForegroundColor Cyan
Write-Host ""

# Navigate to the portable version location
$portablePath = "D:\My_Project\Shakticore\shakticore-photo-restore\dist"

Write-Host "Navigating to: $portablePath" -ForegroundColor Yellow
Set-Location $portablePath

# Check if files exist
$exePath = ".\shakticore-photo-restore.exe"
$modelsPath = ".\models\fast.onnx"

if (-not (Test-Path $exePath)) {
    Write-Host "❌ Error: EXE not found at: $portablePath" -ForegroundColor Red
    pause
    exit 1
}

if (-not (Test-Path $modelsPath)) {
    Write-Host "❌ Error: Models not found!" -ForegroundColor Red
    Write-Host "Expected: $portablePath\models\fast.onnx" -ForegroundColor Yellow
    pause
    exit 1
}

Write-Host "✅ Found EXE" -ForegroundColor Green
Write-Host "✅ Found models" -ForegroundColor Green
Write-Host ""
Write-Host "🎯 Launching app..." -ForegroundColor Cyan

# Launch
Start-Process -FilePath $exePath -WorkingDirectory $portablePath

Write-Host "✅ App launched from portable directory!" -ForegroundColor Green
