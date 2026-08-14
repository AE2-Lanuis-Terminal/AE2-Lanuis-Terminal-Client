# 允许 Android 访问明文 HTTP（模组默认 http://host:8765）
#
# Tauri 生成的 gen/android 在 Release 默认 usesCleartextTraffic=false，
# 浏览器能开网页、App 却 Network Error。android init / 重建 gen 后请再跑本脚本。

param(
  [string]$GradleKts = ""
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if (-not $GradleKts) {
  $GradleKts = Join-Path $root 'src-tauri\gen\android\app\build.gradle.kts'
}
if (-not (Test-Path $GradleKts)) {
  Write-Error "找不到 $GradleKts — 请先 npm run android:init"
}

$text = Get-Content -Raw -LiteralPath $GradleKts
$patched = [regex]::Replace(
  $text,
  'manifestPlaceholders\["usesCleartextTraffic"\]\s*=\s*"false"',
  'manifestPlaceholders["usesCleartextTraffic"] = "true"'
)

if ($patched -eq $text -and $text -notmatch 'usesCleartextTraffic"\]\s*=\s*"true"') {
  # release 块可能未单独设置；在 release 的 getByName 内插入
  $patched = [regex]::Replace(
    $text,
    '(getByName\("release"\)\s*\{\s*)',
    '$1' + "`n            manifestPlaceholders[`"usesCleartextTraffic`"] = `"true`"`n            "
  )
}

if ($patched -eq $text) {
  if ($text -match 'usesCleartextTraffic"\]\s*=\s*"true"') {
    Write-Host "OK: cleartext already enabled in $GradleKts"
    exit 0
  }
  Write-Error "未能自动修补 $GradleKts，请手动将 usesCleartextTraffic 设为 true"
}

Set-Content -LiteralPath $GradleKts -Value $patched -NoNewline
Write-Host "OK: enabled cleartext HTTP in $GradleKts"
