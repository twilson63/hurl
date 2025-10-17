$ErrorActionPreference = 'Stop'

$toolsDir   = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/hurl/hurl/releases/download/v0.1.0/hurl-0.1.0-x86_64-pc-windows-msvc.zip'
$checksum64 = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'

$packageArgs = @{
  packageName   = $env:ChocolateyPackageName
  unzipLocation = $toolsDir
  url64bit      = $url64
  checksum64    = $checksum64
  checksumType64= 'sha256'
}

Install-ChocolateyZipPackage @packageArgs

$installDir = Join-Path $toolsDir 'hurl-0.1.0-x86_64-pc-windows-msvc'
$exePath    = Join-Path $installDir 'hurl.exe'

if (Test-Path $exePath) {
  Install-ChocolateyShortcut -shortcutFilePath "$env:ALLUSERSPROFILE\Microsoft\Windows\Start Menu\Programs\HURL.lnk" `
                             -targetPath $exePath `
                             -description "HURL - Modern HTTP CLI"
}
