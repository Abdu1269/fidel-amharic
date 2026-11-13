# Fidel Amharic — Windows TSF Installation Script
# Must be executed in an elevated (Administrator) PowerShell session.

[CmdletBinding()]
param(
    [string]$SourceDll = "$PSScriptRoot\..\..\target\x86_64-pc-windows-gnu\release\fidel_amharic.dll",
    [string]$InstallDir = "$env:ProgramFiles\Fidel Amharic"
)

$ErrorActionPreference = "Stop"

Write-Host "=== Installing Fidel Amharic Windows TSF Text Service ===" -ForegroundColor Cyan

# Check Administrator privileges
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Error "This script requires Administrator privileges. Please re-run in an elevated PowerShell prompt."
    exit 1
}

if (-not (Test-Path $SourceDll)) {
    # Check alternate release path
    $AltDll = "$PSScriptRoot\..\..\target\release\fidel_amharic.dll"
    if (Test-Path $AltDll) {
        $SourceDll = $AltDll
    } else {
        Write-Error "Source DLL not found at '$SourceDll'. Please build the project with 'cargo build --release -p windows-tsf' first."
        exit 1
    }
}

Write-Host "Target directory: $InstallDir"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$DestDll = Join-Path $InstallDir "fidel_amharic.dll"
Write-Host "Copying $SourceDll -> $DestDll..."
Copy-Item -Path $SourceDll -Destination $DestDll -Force

Write-Host "Registering COM Server and TSF Text Service via regsvr32..."
$regProcess = Start-Process -FilePath "regsvr32.exe" -ArgumentList "/s `"$DestDll`"" -PassThru -Wait

if ($regProcess.ExitCode -eq 0) {
    Write-Host "=== Registration Successful! ===" -ForegroundColor Green
    Write-Host "Fidel Amharic has been registered as an active Windows TSF Input Method."
    Write-Host "You can now switch to Amharic (Power Geez) using Win + Space or the Language Bar."
} else {
    Write-Error "regsvr32 failed with exit code $($regProcess.ExitCode)."
    exit $regProcess.ExitCode
}
