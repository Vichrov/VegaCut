# VegaCut Windows x64 only. No inherited MSI/product identity or signing service.
param([switch]$SkipBuild)
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path "$PSScriptRoot/../..").Path
Push-Location $Root
try {
    $Version = [regex]::Match((Get-Content apps/filmcraft/Cargo.toml -Raw), '(?m)^version = "([^"]+)"').Groups[1].Value
    if (-not $Version) { throw 'Missing app version' }
    $env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS = '-C target-feature=+crt-static'
    $env:FILMCRAFT_REQUIRE_WINRES = '1'
    if (-not $SkipBuild) {
        cargo build --release --locked -p filmcraft -p filmcraft-cli --target x86_64-pc-windows-msvc
        if ($LASTEXITCODE -ne 0) { throw 'Rust build failed' }
    }
    $Stage = Join-Path $Root "target/vegacut-portable/$Version"
    if (Test-Path $Stage) { Remove-Item $Stage -Recurse -Force }
    New-Item $Stage -ItemType Directory -Force | Out-Null
    $Bin = Join-Path $Root 'target/x86_64-pc-windows-msvc/release'
    Copy-Item "$Bin/filmcraft.exe" "$Stage/VegaCut.exe"
    Copy-Item "$Bin/filmcraft-cli.exe" "$Stage/VegaCut-cli.exe"
    foreach ($Name in @('VegaCut.exe','VegaCut-cli.exe')) {
        $Bytes = [IO.File]::ReadAllBytes("$Stage/$Name")
        $Pe = [BitConverter]::ToInt32($Bytes,0x3c)
        if ([BitConverter]::ToUInt16($Bytes,$Pe+4) -ne 0x8664) { throw "Wrong architecture: $Name" }
    }
    Copy-Item LICENSE-MIT, LICENSE-APACHE, NOTICE, README.md, KNOWN_ISSUES.md $Stage
    New-Item "$Stage/font-licences" -ItemType Directory -Force | Out-Null
    Get-ChildItem assets/fonts -Recurse -File | Where-Object { $_.Name -match 'OFL|LICENSE|NOTICE' } | ForEach-Object {
        Copy-Item $_.FullName (Join-Path "$Stage/font-licences" ($_.Directory.Name + '-' + $_.Name))
    }
    Set-Content "$Stage/portable.txt" 'VegaCut portable: settings and recovery are stored in data beside the executable.'
    New-Item dist/release -ItemType Directory -Force | Out-Null
    $Zip = "dist/release/vegacut-$Version-windows-x64-portable.zip"
    Compress-Archive "$Stage/*" $Zip -Force
    $Hash = (Get-FileHash $Zip -Algorithm SHA256).Hash.ToLower()
    Set-Content "$Zip.sha256" "$Hash  $([IO.Path]::GetFileName($Zip))"
    Write-Output "Created $Zip"
} finally { Pop-Location }
