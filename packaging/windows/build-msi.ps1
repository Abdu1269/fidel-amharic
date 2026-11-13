# Fidel Amharic — build unsigned MSI from packaging/windows/installer.wxs
# Callers: .github/workflows/release.yml (windows-latest packaging job)
# Requires WiX Toolset v3 (candle.exe / light.exe) on PATH.

[CmdletBinding()]
param(
    [string]$Version = $env:VERSION,
    [string]$DllPath = $env:DLL_PATH,
    [string]$OutDir = $env:OUT_DIR,
    [string]$WixDir = $env:WIX
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")

if (-not $Version) { $Version = "0.1.0" }
$Version = $Version.TrimStart("v")
if ($Version -notmatch '^\d+\.\d+\.\d+') {
    Write-Error "VERSION must look like 0.1.0 (got '$Version')"
}

if (-not $OutDir) { $OutDir = Join-Path $RepoRoot "dist" }
New-Item -ItemType Directory -Path $OutDir -Force | Out-Null

if (-not $DllPath) {
    $candidates = @(
        (Join-Path $RepoRoot "target\x86_64-pc-windows-gnu\release\fidel_amharic.dll"),
        (Join-Path $RepoRoot "target\x86_64-pc-windows-msvc\release\fidel_amharic.dll"),
        (Join-Path $RepoRoot "target\release\fidel_amharic.dll"),
        (Join-Path $ScriptDir "fidel_amharic.dll")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $DllPath = (Resolve-Path $c).Path; break }
    }
}

if (-not $DllPath -or -not (Test-Path $DllPath)) {
    Write-Error "fidel_amharic.dll not found. Set -DllPath or build windows-tsf first."
}

$StageDir = Join-Path $RepoRoot "target\wix-stage"
Remove-Item -Recurse -Force $StageDir -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $StageDir -Force | Out-Null
Copy-Item $DllPath (Join-Path $StageDir "fidel_amharic.dll") -Force

$WxsSource = Join-Path $ScriptDir "installer.wxs"
$WxsBuilt = Join-Path $StageDir "installer.wxs"
$wxsText = (Get-Content $WxsSource -Raw) -replace 'Version="0\.1\.0"', "Version=`"$Version`""
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllText($WxsBuilt, $wxsText, $utf8NoBom)

$Candle = "candle.exe"
$Light = "light.exe"
if ($WixDir) {
    $Candle = Join-Path $WixDir "bin\candle.exe"
    $Light = Join-Path $WixDir "bin\light.exe"
}

if (-not (Get-Command $Candle -ErrorAction SilentlyContinue) -and -not (Test-Path $Candle)) {
    Write-Error "WiX candle.exe not found. Install WiX Toolset v3 or set WIX."
}
if (-not (Get-Command $Light -ErrorAction SilentlyContinue) -and -not (Test-Path $Light)) {
    Write-Error "WiX light.exe not found. Install WiX Toolset v3 or set WIX."
}

Push-Location $StageDir
try {
    & $Candle -nologo "-dDllDir=$StageDir" -out "installer.wixobj" "installer.wxs"
    if ($LASTEXITCODE -ne 0) { throw "candle failed" }

    $MsiName = "fidel-amharic-$Version-windows-x86_64.msi"
    $MsiPath = Join-Path $OutDir $MsiName
    & $Light -nologo -out $MsiPath "installer.wixobj"
    if ($LASTEXITCODE -ne 0) { throw "light failed" }

    Write-Host "Built $MsiPath"
} finally {
    Pop-Location
}
