<#
.SYNOPSIS
Installer sekali klik untuk FinFlow (Windows).

.DESCRIPTION
Mengunduh ZIP rilis terbaru dari GitHub Releases, mengekstraknya ke
%LOCALAPPDATA%\FinFlow, lalu membuat shortcut di Start Menu.

.EXAMPLE
powershell -ExecutionPolicy Bypass -File .\install.ps1 -Repo owner/finance-flow

.EXAMPLE
irm https://raw.githubusercontent.com/l3ockydev/finance-flow/main/install.ps1 | iex
#>
param(
    [string]$Repo = "",
    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA "FinFlow")
)

$ErrorActionPreference = "Stop"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

# --- Tentukan owner/repo -----------------------------------------------------
# Default ke repo resmi agar skrip bisa dijalankan langsung dari URL
# (tanpa clone), mis. via: irm https://raw.githubusercontent.com/.../install.ps1 | iex
if (-not $Repo -and $PSScriptRoot) {
    $remote = & git -C $PSScriptRoot remote get-url origin 2>$null
    if ($LASTEXITCODE -eq 0 -and $remote -match 'github\.com[:/]([^/]+)/([^/\s]+?)(?:\.git)?\s*$') {
        $Repo = "$($Matches[1])/$($Matches[2])"
    }
}
if (-not $Repo) { $Repo = "l3ockydev/finance-flow" }
if ($Repo -notmatch '^[^/\s]+/[^/\s]+$') {
    throw "Repo GitHub belum diketahui / GitHub repo unknown.`nJalankan / Run: .\install.ps1 -Repo owner/repo (mis. budi/finance-flow)"
}

Write-Host "FinFlow installer - repo: $Repo"

# --- Ambil aset rilis terbaru ------------------------------------------------
$headers = @{ "User-Agent" = "FinFlow-Installer" }
try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -Headers $headers
} catch {
    throw "Gagal menghubungi GitHub / Cannot reach GitHub. Periksa koneksi/nama repo.`n$_"
}

$asset = $release.assets |
    Where-Object { $_.name -match 'windows.*x86_64.*\.zip$' -or $_.name -match 'win.*\.zip$' } |
    Select-Object -First 1
if (-not $asset) {
    throw "Rilis terbaru / Latest release ($($release.tag_name)) belum punya ZIP. Hubungi maintainer."
}
Write-Host "Mengunduh / Downloading $($asset.name) ($([math]::Round($asset.size / 1MB, 1)) MB)..."

# --- Unduh & ekstrak (data di projects\ tidak dihapus) -----------------------
$tmp = Join-Path ([IO.Path]::GetTempPath()) ("finflow-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $tmp | Out-Null
try {
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile "$tmp\finflow.zip" -Headers $headers
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Expand-Archive -Path "$tmp\finflow.zip" -DestinationPath $InstallDir -Force

    $exe = Join-Path $InstallDir "finance_flow.exe"
    if (-not (Test-Path $exe)) {
        throw "finance_flow.exe tidak ditemukan / not found in the release ZIP."
    }

    # --- Shortcut Start Menu ------------------------------------------------
    $startMenu = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs"
    New-Item -ItemType Directory -Force -Path $startMenu | Out-Null
    $shortcutPath = Join-Path $startMenu "FinFlow.lnk"
    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut($shortcutPath)
    $shortcut.TargetPath = $exe
    $shortcut.WorkingDirectory = $InstallDir
    $shortcut.IconLocation = "$exe,0"
    $shortcut.Description = "FinFlow - Keuangan Pribadi"
    $shortcut.Save()

    Write-Host ""
    Write-Host "FinFlow berhasil terpasang / installed successfully."
    Write-Host "  Lokasi / Location: $InstallDir"
    Write-Host "  Shortcut         : $shortcutPath"
    Write-Host "  Data             : $(Join-Path $InstallDir 'projects')"
} finally {
    Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
