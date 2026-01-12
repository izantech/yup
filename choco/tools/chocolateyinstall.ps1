$ErrorActionPreference = 'Stop'

$packageName = 'yup'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# URLs and checksums are replaced during release
$url64 = '{{URL_X64}}'
$urlArm64 = '{{URL_ARM64}}'
$checksum64 = '{{CHECKSUM_X64}}'
$checksumArm64 = '{{CHECKSUM_ARM64}}'

$packageArgs = @{
  packageName    = $packageName
  unzipLocation  = $toolsDir
  url64bit       = $url64
  checksum64     = $checksum64
  checksumType64 = 'sha256'
}

# Use ARM64 binary on ARM64 Windows
if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') {
  $packageArgs['url64bit'] = $urlArm64
  $packageArgs['checksum64'] = $checksumArm64
}

Install-ChocolateyZipPackage @packageArgs
