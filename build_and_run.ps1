$ErrorActionPreference = "Stop"

# Zapamiętaj katalog skryptu, żeby używać pełnych ścieżek
$scriptPath = $PSScriptRoot

# 1. Build the project (RELEASE zamiast debug - ważne dla UEFI)
Write-Host "Building project (Release)..." -ForegroundColor Cyan
cargo build --target x86_64-unknown-uefi --release

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
    exit 1
}

# 2. Prepare the ESP directory structure
$espDir = Join-Path $scriptPath "esp"
$bootDir = Join-Path $espDir "efi\boot"

if (-not (Test-Path $bootDir)) {
    Write-Host "Creating ESP directory structure..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Force -Path $bootDir | Out-Null
}

# 3. Copy executable (Ścieżka zmieniona na 'release')
$sourceExe = Join-Path $scriptPath "target\x86_64-unknown-uefi\release\uefi_game.efi"
$destExe = Join-Path $bootDir "bootx64.efi"

Write-Host "Copying bootx64.efi..." -ForegroundColor Cyan
Copy-Item -Path $sourceExe -Destination $destExe -Force

# 4. Run QEMU
# Używamy pflash zamiast -bios, bo to u Ciebie zadziałało
# Używamy pełnej ścieżki do bios.fd ($biosPath)
$biosPath = Join-Path $scriptPath "bios.fd"

Write-Host "Starting QEMU..." -ForegroundColor Green

# Sprawdzenie czy bios istnieje przed startem, żeby uniknąć mylących błędów QEMU
if (-not (Test-Path $biosPath)) {
    Write-Error "Nie znaleziono pliku bios.fd w folderze projektu!"
    exit 1
}

& qemu-system-x86_64 `
    -nodefaults `
    -vga std `
    -drive if=pflash,format=raw,file="$biosPath",readonly=on `
    -drive format=raw,file=fat:rw:"$espDir"