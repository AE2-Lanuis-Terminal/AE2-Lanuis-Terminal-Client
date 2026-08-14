# 贡献指南（Client）

- **禁止直推 `main`**，只接受 PR
- 日常 PR：更新 `CHANGELOG.md` 的 `## [Unreleased]`
- **发行 PR**：收入 `## [X.Y.Z]`，bump `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`；声明依赖的 Web tag（通常 `vX.Y.Z`）

发版细节：[RELEASE.md](RELEASE.md)。开发细节：[DEVELOPMENT.md](DEVELOPMENT.md)。
