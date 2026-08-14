# 本地开发（Client）

## 依赖 Web

`web/` 为 git submodule（见 [.gitmodules](../.gitmodules)）。`package.json` 使用 `--prefix web`，`tauri.conf.json` 的 `frontendDist` 为 `../web/dist`。

| 场景 | 做法 |
|------|------|
| 正式 | `git clone --recurse-submodules …` 或 `git submodule update --init` |
| 本机复用兄弟 Web 仓 | `.\scripts\link-web.ps1`（Junction → `web/`） |
| CI | `actions/checkout` + `submodules: recursive` |

发版时把 `web` submodule 指针钉到 Web tag `vX.Y.Z`。

## 平台拆分

- Rust：`#[cfg(desktop)]` 托盘 / 设置窗 / `window_api`；`#[cfg(mobile)]` 仅连接配置等
- 前端：Web 仓 `platform.ts` 的 `isDesktopChrome()` / `isMobileShell()`；**勿**依赖构建期 `TAURI_*`
- 合成完成：`tauri-plugin-notification`（与 Jobs「完成推送」共用）；纯 Web 只有应用内 Toast

## 命令

| 命令 | 说明 |
|------|------|
| `npm run tauri:dev` | 桌面开发 |
| `npm run tauri:build` | Windows NSIS |
| `npm run android:init` / `dev` / `build` | Android（详见 [ANDROID.md](ANDROID.md)） |

## 范围

- 目标：Windows 桌面 + Android
- 非目标：macOS / Linux 桌面发行；iOS 后续可选
