$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
& $cargo test --locked --lib --no-default-features
if ($LASTEXITCODE -ne 0) { throw 'Tests failed' }
& $cargo build --locked --release -j 2
if ($LASTEXITCODE -ne 0) { throw 'Rust build failed' }
& $cargo test --locked --release --bin sonarqube-manager -j 2
if ($LASTEXITCODE -ne 0) { throw 'Bundled icon verification failed' }
$compiler = Get-Command ISCC.exe -ErrorAction SilentlyContinue
$iscc = if ($compiler) { $compiler.Source } elseif (Test-Path tools\inno\ISCC.exe) { (Resolve-Path tools\inno\ISCC.exe).Path } else { 'C:\Program Files (x86)\Inno Setup 6\ISCC.exe' }
if (!(Test-Path -LiteralPath $iscc)) { throw 'Install Inno Setup 6, or add ISCC.exe to PATH. Rust executable is built in target\release.' }
& $iscc installer\sonarqube-manager.iss
if ($LASTEXITCODE -ne 0) { throw 'Installer compilation failed' }
