# Client 发版

消费 **Web 仓最新发行 tag**（`vX.Y.Z`）：发行 PR 将 `web` submodule 钉到该 tag，再构建 Windows NSIS 与 Android APK。

总流程：[主仓 docs/RELEASE.md](https://github.com/Lexcubia/AE2-Lanuis-Terminal/blob/main/docs/RELEASE.md)。

## 发行 PR 清单

1. 确认 Web 已存在 tag `vX.Y.Z`
2. `git -C web fetch --tags && git -C web checkout vX.Y.Z`（或更新 submodule 指针）
3. CHANGELOG 收入 `## [X.Y.Z]`，并写明 `Depends-on-web: vX.Y.Z`
4. bump：`package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`
5. 合入 `main` → `release.yml` 按 submodule 指针打包

## 本地 Web 依赖

```bash
git submodule update --init
# 或本机 Junction：
# .\scripts\link-web.ps1 -WebPath "E:\GIT\AE2-Lanuis-Terminal-Web"
```

然后：

```bash
npm install
npm run tauri:build      # Windows NSIS
npm run android:build    # 需 SDK/NDK；Windows 需 Developer Mode（见 docs/ANDROID.md）
```

## CI 说明

- 日常 `ci.yml`：submodule `web`（随仓库指针，通常跟进 main）→ 烟测
- 发行 `release.yml`：使用发行 PR 钉死的 submodule 指针；可选 `web_tag` 覆盖检出 → Win NSIS + Linux APK → artifact；`PUBLISH=false` 时不创建 GitHub Release
