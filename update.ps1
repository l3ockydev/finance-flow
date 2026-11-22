<#
.SYNOPSIS
Pembaruan otomatis FinFlow dari GitHub Releases.

.DESCRIPTION
Dijalankan oleh finance_flow.exe saat aplikasi dibuka. Script ini memeriksa
rilis terbaru (maksimal sekali per jam), mengunduh ZIP jika ada versi baru,
lalu menunggu aplikasi ditutup sebelum mengganti executable. Sesi yang sedang
berjalan tidak terganggu. Semua kegagalan diabaikan secara senyap agar
aplikasi tetap bisa dipakai tanpa internet.

Launched by finance_flow.exe on startup. Checks the latest GitHub release
(at most once per hour), downloads the ZIP when a newer version exists, and
waits for the app to close before replacing the executable. The active session
is never interrupted. Failures are ignored silently so the app stays offline-safe.
#>
param(
    [string]$AppDir = $PSScriptRoot,
    [int]$ParentPid = 0,
    [string]$CurrentVersion = "0.1.0",
    [string]$Repo = ""
)

$ErrorActionPreference = "Stop"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

function Write-State($data, $path) {
    try { ($data | ConvertTo-Json -Compress) | Set-Content -Path $path -Encoding UTF8 } catch { }
}

$stateDir = Join-Path $env:LOCALAPPDATA "FinFlow"
$stateFile = Join-Path $stateDir "update_state.json"
$appliedFile = Join-Path $stateDir "update_applied.txt"
$tmp = $null

try {
    New-Item -ItemType Directory -Force -Path $stateDir | Out-Null

    # --- Tentukan repository -------------------------------------------------
    if (-not $Repo) {
        $config = Join-Path $AppDir "update.json"
        if (Test-Path $config) {
            try { $Repo = (Get-Content $config -Raw | ConvertFrom-Json).repo } catch { }
        }
    }
    if ($Repo -notmatch '^[^/\s]+/[^/\s]+$') { exit 0 }

    # --- Batasi pemeriksaan menjadi sekali per jam ---------------------------
    $lastCheck = 0
    if (Test-Path $stateFile) {
        try { $lastCheck = [double](Get-Content $stateFile -Raw | ConvertFrom-Json).last_check_epoch } catch { }
    }
    $now = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds()
    if (($now - $lastCheck) -lt 3600) { exit 0 }
    Write-State @{ last_check_epoch = $now } $stateFile

    # --- Cek rilis terbaru ---------------------------------------------------
    $headers = @{ "User-Agent" = "FinFlow-AutoUpdate" }
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -Headers $headers -TimeoutSec 20
    $newVersion = ($release.tag_name -replace '^v', '')

    try {
        if ([version]$newVersion -le [version]$CurrentVersion) { exit 0 }
    } catch {
        if ($newVersion -eq $CurrentVersion) { exit 0 }
    }

    $asset = $release.assets |
        Where-Object { $_.name -match 'windows.*x86_64.*\.zip$' } |
        Select-Object -First 1
    if (-not $asset) { exit 0 }

    # --- Unduh & ekstrak ke folder sementara ---------------------------------
    $tmp = Join-Path ([IO.Path]::GetTempPath()) ("ffupdate-" + [guid]::NewGuid().ToString("N"))
    $zip = Join-Path $tmp "update.zip"
    $out = Join-Path $tmp "files"
    New-Item -ItemType Directory -Force -Path $out | Out-Null
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $zip -Headers $headers -TimeoutSec 120
    Expand-Archive -Path $zip -DestinationPath $out -Force

    $newExe = Join-Path $out "finance_flow.exe"
    $oldExe = Join-Path $AppDir "finance_flow.exe"
    if (-not (Test-Path $newExe) -or -not (Test-Path $oldExe)) { exit 0 }

    # --- Tunggu aplikasi ditutup (maksimal 2 menit) --------------------------
    if ($ParentPid -gt 0) {
        for ($i = 0; $i -lt 240; $i++) {
            if (-not (Get-Process -Id $ParentPid -ErrorAction SilentlyContinue)) { break }
            Start-Sleep -Milliseconds 500
        }
        if (Get-Process -Id $ParentPid -ErrorAction SilentlyContinue) { exit 0 }
    }

    # --- Pasang: ganti executable, simpan penanda untuk notifikasi -----------
    Copy-Item $newExe $oldExe -Force
    Set-Content -Path $appliedFile -Value $newVersion -Encoding UTF8
} catch {
    # Update gagal (offline, rate-limit, dll.) — aplikasi tetap normal.
} finally {
    if ($tmp -and (Test-Path $tmp)) { Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue }
}
