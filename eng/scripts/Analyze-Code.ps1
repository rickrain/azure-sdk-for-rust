#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [switch]$CheckWasm = $true,
  [switch]$SkipPackageAnalysis
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')

Write-Host @"
Analyzing code with
    RUSTFLAGS: '${env:RUSTFLAGS}'
    RUSTDOCFLAGS: '${env:RUSTDOCFLAGS}'
"@

if ($CheckWasm) {
  Invoke-LoggedCommand "rustup target add wasm32-unknown-unknown"
}

Invoke-LoggedCommand "cargo check --package azure_core --all-features --all-targets --keep-going"

Invoke-LoggedCommand "cargo fmt --all -- --check"

Invoke-LoggedCommand "cargo clippy --workspace --all-features --all-targets --keep-going --no-deps"

if ($CheckWasm) {
  Invoke-LoggedCommand "cargo clippy --target=wasm32-unknown-unknown --workspace --keep-going --no-deps"
}

Invoke-LoggedCommand "cargo doc --workspace --no-deps --all-features"

# Verify package dependencies

# BUGBUG: https://github.com/Azure/azure-sdk-for-rust/issues/2186
# $verifyDependenciesScript = Join-Path $RepoRoot 'eng' 'scripts' 'verify-dependencies.rs' -Resolve
#
# if (!$SkipPackageAnalysis) {
#   if (!(Test-Path $PackageInfoDirectory)) {
#     Write-Error "Package info path '$PackageInfoDirectory' does not exist."
#     exit 1
#   }
#
#   $packagesToTest = Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse
#   | Get-Content -Raw
#   | ConvertFrom-Json
#
#   Push-Location
#   try {
#     foreach ($package in $packagesToTest) {
#       Set-Location (Join-Path $RepoRoot $package.DirectoryPath)
#       Write-Host "Analyzing package: '$($package.Name)' in directory: '$($package.DirectoryPath)'`n"
#       Invoke-LoggedCommand "cargo +nightly -Zscript $verifyDependenciesScript"
#     }
#   }
#   finally {
#     Pop-Location
#   }
# }
