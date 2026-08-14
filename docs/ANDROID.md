# Android（Tauri Mobile）

## 前置

- Android Studio、SDK、NDK、JDK 17+
- 环境变量：`ANDROID_HOME`、可选 `NDK_HOME`（见 https://v2.tauri.app/start/prerequisites/）
- `web/` submodule 已 init（或先跑 `scripts/link-web.ps1`）

## 初始化

在已能 `npm run tauri:dev`（Windows）的前提下：

```bash
npm install
npm run android:init   # 生成 src-tauri/gen/android（gitignore）
npm run android:dev
```

打包：`npm run android:build`。

## Windows 本机注意（符号链接）

`tauri android build` 会把 `lib*.so` **符号链接**到 `gen/android/.../jniLibs/`。若报：

`Creation symbolic link is not allowed for this system`

请任选其一：

1. 打开 **Windows 开发者模式**（设置 → 系统 → 开发者选项），再重跑；或
2. 手动硬链接/拷贝 `.so` 后 Gradle 组装（CI 使用 **Ubuntu** runner，无此问题）：

```powershell
$src = "src-tauri\target\aarch64-linux-android\release\libae2_lanuis_client_lib.so"
$dest = "src-tauri\gen\android\app\src\main\jniLibs\arm64-v8a\libae2_lanuis_client_lib.so"
New-Item -ItemType Directory -Force -Path (Split-Path $dest) | Out-Null
if (Test-Path $dest) { Remove-Item -Force $dest }
cmd /c "mklink /H `"$dest`" `"$src`""
cd src-tauri\gen\android
.\gradlew.bat :app:assembleArm64Release -x rustBuildArm64Release -x rustBuildUniversalRelease
```

正式发布以 CI Linux 产物为准；本机 debug 签名 APK 仅供侧载验证。

## 明文 HTTP 与登录 Network Error

模组默认 `http://…:8765`。仅开 `usesCleartextTraffic` 往往不够：Android WebView 仍可能拦跨源请求。

本仓做法：

1. `scripts/enable-android-cleartext.ps1`（`android init` 后）
2. 前端在 Tauri 下用 `@tauri-apps/plugin-http`（Rust）发 API，不经 WebView `fetch`

登录填 **HTTP 端口**（默认 `8765`），不是 MC `25565`，也不是独立 WebSocket 端口（你配置成 `8766` 时，WS 会在登录后由 health 自动发现）。

建议把 `websocket.port` 设为 `0`（与 HTTP 同端口），少开一个防火墙端口。

## 与桌面差异

- Rust：`#[cfg(desktop)]` 托盘 / 设置窗；移动端仅连接配置与通知
- 前端：`isMobileShell()` / `preferTouchTargets()` / 安全区
- 不发行 macOS/Linux 桌面；iOS 未纳入本阶段
- 合成完成：系统通知（需用户授权「完成推送」）
