# Fidel Amharic — Windows TSF Installation Script
# Must be executed in an elevated (Administrator) PowerShell session.
# Works from a release zip (DLL next to this script) or from a source checkout.
#
# Callers: docs/os-installation.md, .github/workflows/release.yml (copied into release zip),
#          packaging/windows/installer.wxs (related MSI path).
# No data-file schema beyond copying fidel_amharic.dll and regsvr32 registration.
# User instruction: "Implement the plan as specified" (IME click-install: 1B unsigned, 2A IME only).

[CmdletBinding()]
param(
    [string]$SourceDll = "",
    [string]$InstallDir = "$env:ProgramFiles\Fidel Amharic"
)

$ErrorActionPreference = "Stop"

Write-Host "=== Installing Fidel Amharic Windows TSF Text Service ===" -ForegroundColor Cyan

$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Error "This script requires Administrator privileges. Please re-run in an elevated PowerShell prompt."
    exit 1
}

function Resolve-SourceDll {
    param([string]$Explicit)

    if ($Explicit -and (Test-Path $Explicit)) {
        return (Resolve-Path $Explicit).Path
    }

    $candidates = @(
        (Join-Path $PSScriptRoot "fidel_amharic.dll"),
        (Join-Path $PSScriptRoot "..\..\target\x86_64-pc-windows-gnu\release\fidel_amharic.dll"),
        (Join-Path $PSScriptRoot "..\..\target\x86_64-pc-windows-msvc\release\fidel_amharic.dll"),
        (Join-Path $PSScriptRoot "..\..\target\release\fidel_amharic.dll")
    )

    foreach ($path in $candidates) {
        if (Test-Path $path) {
            return (Resolve-Path $path).Path
        }
    }

    return $null
}

$ResolvedDll = Resolve-SourceDll -Explicit $SourceDll
if (-not $ResolvedDll) {
    Write-Error "fidel_amharic.dll not found next to this script or under target/. Build with 'cargo build --release -p windows-tsf' or use a release zip."
    exit 1
}

Write-Host "Source DLL: $ResolvedDll"
Write-Host "Target directory: $InstallDir"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$DestDll = Join-Path $InstallDir "fidel_amharic.dll"
Write-Host "Copying $ResolvedDll -> $DestDll..."
Copy-Item -Path $ResolvedDll -Destination $DestDll -Force

Write-Host "Registering COM Server and TSF Text Service via regsvr32..."
$regProcess = Start-Process -FilePath "regsvr32.exe" -ArgumentList "/s `"$DestDll`"" -PassThru -Wait

if ($regProcess.ExitCode -eq 0) {
    Write-Host "=== Registration Successful! ===" -ForegroundColor Green
    Write-Host "Fidel Amharic has been registered as an active Windows TSF Input Method."
    Write-Host "You can now switch to Amharic (Power Geez) using Win + Space or the Language Bar."
    Write-Host ""
    Write-Host "Note: This build is unsigned. If SmartScreen warns, choose More info -> Run anyway."
} else {
    Write-Error "regsvr32 failed with exit code $($regProcess.ExitCode)."
    exit $regProcess.ExitCode
}
