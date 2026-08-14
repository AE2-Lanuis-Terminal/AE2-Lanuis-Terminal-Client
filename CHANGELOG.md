# Changelog

本仓库遵循 [Keep a Changelog](https://keepachangelog.com/)，版本号遵循 [SemVer](https://semver.org/)。

Client 发版须声明依赖的 **Web tag**（如 `v0.1.0`）。总流程见 [docs/RELEASE.md](docs/RELEASE.md)。

## [Unreleased]

### Fixed

- Android 登录 Network Error：Tauri 改走 `plugin-http`（Rust）请求模组 HTTP，并允许明文 cleartext

### Added

- CI / Release workflow 脚手架（默认只上传 artifact）
- 合成完成系统通知（`tauri-plugin-notification`）
- 发版文档、`scripts/link-web.ps1`、`scripts/enable-android-cleartext.ps1`
- 根目录 MIT `LICENSE`

## [0.1.0] - 2026-08-14

### Added

- 初始 Client：Windows 桌面 + Android（依赖 Web UI）
