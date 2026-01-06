# Post-Install Script for MSI
# This script runs after MSI installation to copy models

$installPath = "C:\Program Files\shakticore-photo-restore"
$modelsSource = Join-Path $PSScriptRoot "models"
$modelsTarget = Join-Path $installPath "models"

Write-Host "ShaktiCore Photo Restore - Post-Install Setup" -ForegroundColor Cyan
Write-Host ""

# Check if installation directory exists
if (-not (Test-Path $installPath)) {
    Write-Host "❌ Installation directory not found: $installPath" -ForegroundColor Red
    Write-Host "Please install the MSI first." -ForegroundColor Yellow
    pause
    exit 1
}

# Check if models source exists
if (-not (Test-Path $modelsSource)) {
    Write-Host "❌ Models folder not found: $modelsSource" -ForegroundColor Red
    Write-Host "Please run this script from the extracted ZIP folder." -ForegroundColor Yellow
    pause
    exit 1
}

# Copy models
Write-Host "📦 Copying models to installation directory..." -ForegroundColor Yellow
try {
    Copy-Item -Path $modelsSource -Destination $installPath -Recurse -Force
    Write-Host "✅ Models copied successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Models installed to: $modelsTarget" -ForegroundColor White
    Write-Host ""
    Write-Host "✅ Setup complete! You can now launch ShaktiCore from Start Menu." -ForegroundColor Green
}
catch {
    Write-Host "❌ Failed to copy models: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Manual steps:" -ForegroundColor Yellow
    Write-Host "1. Copy the 'models' folder" -ForegroundColor White
    Write-Host "2. Paste it to: $installPath" -ForegroundColor White
}

Write-Host ""
pause
