# Fidel Amharic — Windows TSF Uninstallation Script
# Must be executed in an elevated (Administrator) PowerShell session.

[CmdletBinding()]
param(
    [string]$InstallDir = "$env:ProgramFiles\Fidel Amharic"
)

$ErrorActionPreference = "Stop"

Write-Host "=== Unregistering Fidel Amharic Windows TSF Text Service ===" -ForegroundColor Cyan

$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Error "This script requires Administrator privileges. Please re-run in an elevated PowerShell prompt."
    exit 1
}

$DestDll = Join-Path $InstallDir "fidel_amharic.dll"
if (Test-Path $DestDll) {
    Write-Host "Unregistering COM Server and TSF Text Service via regsvr32 /u..."
    $regProcess = Start-Process -FilePath "regsvr32.exe" -ArgumentList "/u /s `"$DestDll`"" -PassThru -Wait
    if ($regProcess.ExitCode -ne 0) {
        Write-Warning "regsvr32 /u exited with code $($regProcess.ExitCode)."
    }
}

if (Test-Path $InstallDir) {
    Write-Host "Removing installed files from $InstallDir..."
    Remove-Item -Path $InstallDir -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Host "=== Uninstallation Complete! ===" -ForegroundColor Green
