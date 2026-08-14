param(
  [string]$WebPath = ""
)

$ErrorActionPreference = 'Stop'
$clientRoot = Split-Path -Parent $PSScriptRoot
$linkPath = Join-Path $clientRoot 'web'

if (-not $WebPath) {
  $candidates = @(
    (Join-Path (Split-Path -Parent $clientRoot) 'AE2-Lanuis-Terminal-Web'),
    (Join-Path $clientRoot '..\AE2-Lanuis-Terminal-Web')
  )
  foreach ($c in $candidates) {
    $resolved = [System.IO.Path]::GetFullPath($c)
    if (Test-Path (Join-Path $resolved 'package.json')) {
      $WebPath = $resolved
      break
    }
  }
}

if (-not $WebPath -or -not (Test-Path (Join-Path $WebPath 'package.json'))) {
  Write-Error "Web 仓未找到。请传入 -WebPath，例如: .\scripts\link-web.ps1 -WebPath E:\GIT\AE2-Lanuis-Terminal-Web"
}

$WebPath = [System.IO.Path]::GetFullPath($WebPath)
$linkPath = [System.IO.Path]::GetFullPath($linkPath)

if (Test-Path $linkPath) {
  $item = Get-Item $linkPath
  if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) {
    Write-Host "已存在 junction: $linkPath -> $($item.Target)"
    exit 0
  }
  Write-Error "路径已存在且不是 junction: $linkPath（请先 git submodule update --init，或删除后重试）"
}

cmd /c "mklink /J `"$linkPath`" `"$WebPath`""
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "OK: $linkPath => $WebPath"
Write-Host "提示：正式贡献请用 git submodule；本脚本仅本机 Junction 便捷开发。"
