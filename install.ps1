# AeroFlow Installation Script (Windows PowerShell)
# Usage: iwr https://get.aeroflow.dev/install.ps1 -useb | iex

$ErrorActionPreference = "Stop"

$AEROFLOW_VERSION = "1.0.0"
$INSTALL_DIR = "$env:LOCALAPPDATA\aeroflow\bin"
$DOWNLOAD_URL = "https://github.com/aeroflow/aeroflow/releases/download/v$AEROFLOW_VERSION/aeroflow-windows-x86_64.exe"

Write-Host "🌀 AeroFlow Installer v$AEROFLOW_VERSION" -ForegroundColor Cyan
Write-Host ""

# Create installation directory
if (-not (Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR | Out-Null
}

Write-Host "📥 Downloading AeroFlow..." -ForegroundColor Yellow

# Download binary
$TEMP_PATH = "$env:TEMP\aeroflow.exe"
try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TEMP_PATH -UseBasicParsing
} catch {
    Write-Host "❌ Download failed: $_" -ForegroundColor Red
    exit 1
}

# Move to installation directory
$FINAL_PATH = "$INSTALL_DIR\aeroflow.exe"
Move-Item -Path $TEMP_PATH -Destination $FINAL_PATH -Force

# Add to PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$INSTALL_DIR*") {
    Write-Host "🔧 Adding to PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$UserPath;$INSTALL_DIR",
        "User"
    )
    $env:Path += ";$INSTALL_DIR"
}

# Verify installation
Write-Host ""
Write-Host "✅ AeroFlow installed successfully!" -ForegroundColor Green
Write-Host ""

& $FINAL_PATH --version

Write-Host ""
Write-Host "📚 Next steps:" -ForegroundColor Cyan
Write-Host "  aeroflow init myapp"
Write-Host "  cd myapp"
Write-Host "  aeroflow dev"
Write-Host ""
Write-Host "📖 Documentation: https://docs.aeroflow.dev" -ForegroundColor Cyan
Write-Host ""
Write-Host "⚠️  Restart your terminal to use 'aeroflow' command globally" -ForegroundColor Yellow
