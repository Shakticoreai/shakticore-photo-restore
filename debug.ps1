# Debug Script - Check Model and Test Processing

Write-Host "Checking model file..." -ForegroundColor Cyan

# Check if model exists
$modelPath = "models\fast.onnx"
if (Test-Path $modelPath) {
    $size = [math]::Round((Get-Item $modelPath).Length / 1MB, 2)
    Write-Host "✅ Model found: $modelPath ($size MB)" -ForegroundColor Green
}
else {
    Write-Host "❌ Model NOT found: $modelPath" -ForegroundColor Red
    Write-Host "Creating models directory..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Force -Path "models" | Out-Null
    
    # Copy from source
    $sourcePath = "..\..\shakticore_ai\runtime\dist\shakticore_complete\models\upscaler\realesrgan_x4plus_compact.onnx"
    if (Test-Path $sourcePath) {
        Write-Host "Copying model from: $sourcePath" -ForegroundColor Yellow
        Copy-Item $sourcePath $modelPath
        Write-Host "✅ Model copied" -ForegroundColor Green
    }
    else {
        Write-Host "❌ Source model not found: $sourcePath" -ForegroundColor Red
    }
}

Write-Host "`nRunning app in dev mode to see console output..." -ForegroundColor Cyan
Write-Host "Press Ctrl+C to stop" -ForegroundColor Gray
Write-Host ""

# Run in dev mode to see logs
npm run tauri dev
