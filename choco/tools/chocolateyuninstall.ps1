$ErrorActionPreference = 'Stop'

$packageName = 'yup'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove the executable
$exePath = Join-Path $toolsDir 'yup.exe'
if (Test-Path $exePath) {
  Remove-Item $exePath -Force
}
