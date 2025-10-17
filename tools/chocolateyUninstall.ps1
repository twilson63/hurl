$ErrorActionPreference = 'Stop'

Remove-Item "$env:ALLUSERSPROFILE\Microsoft\Windows\Start Menu\Programs\HURL.lnk" -Force -ErrorAction SilentlyContinue

$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$installDir = Join-Path $toolsDir 'hurl-0.1.0-x86_64-pc-windows-msvc'

if (Test-Path $installDir) {
  Remove-Item $installDir -Recurse -Force -ErrorAction SilentlyContinue
}
