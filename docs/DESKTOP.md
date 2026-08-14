# Desktop（Windows）— 开发说明

使用者安装与登录见根目录 [README.md](../README.md)。下文面向从源码改桌面壳的开发者。

- `tauri.conf.json`：`decorations: false`，拖拽见 Web `bindTauriWindowDragRegion`
- 托盘与关闭行为：Rust `tray.rs` + 设置「关闭按钮」
- 打包：`npm run tauri:build` → NSIS
- 不做 macOS / Linux 桌面发行

另见 [DEVELOPMENT.md](DEVELOPMENT.md)、[RELEASE.md](RELEASE.md)。
