# Client 发版

消费 **Web 仓最新发行 tag**（`vX.Y.Z`）构建 Windows NSIS 与 Android APK。

总流程：[主仓 docs/RELEASE.md](https://github.com/AE2-Lanuis-Terminal/AE2-Lanuis-Terminal/blob/main/docs/RELEASE.md)。

## 发行 PR 清单

1. 确认 Web 已存在 tag `vX.Y.Z`
2. CHANGELOG 收入 `## [X.Y.Z]`，并写明 `Depends-on-web: vX.Y.Z`
3. bump：`package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`
4. 合入 `main` → `release.yml` 检出 Web@tag 后打包

## 本地 Web 依赖

```powershell
# 若兄弟目录不叫 web：
.\scripts\link-web.ps1
# 或
.\scripts\link-web.ps1 -WebPath "E:\GIT\AE2-Lanuis-Terminal-Web"
```

然后：

```bash
npm install
npm run tauri:build      # Windows NSIS
npm run android:build    # 需 SDK/NDK；Windows 需 Developer Mode（见 docs/ANDROID.md）
```

## CI 说明

- 日常 `ci.yml`：checkout Web@`main` → 烟测
- 发行 `release.yml`：输入/解析 Web tag → Win NSIS + Linux APK → artifact；`PUBLISH=false` 时不创建 GitHub Release
