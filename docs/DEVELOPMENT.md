# 本地开发（Client）

## 依赖 Web

构建与开发都通过 `../web`（`package.json` 的 `npm run --prefix ../web …`，`tauri.conf.json` 的 `frontendDist`）。

| 场景 | 做法 |
|------|------|
| 主仓 `--recurse-submodules` | 在 `client/` 下开发，Web 在 `../web` |
| 单独 clone | 兄弟目录命名为 `web`，或 `.\scripts\link-web.ps1` |
| CI | 额外 checkout Web 仓到 `web/`（与 `client/` 并列） |

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
