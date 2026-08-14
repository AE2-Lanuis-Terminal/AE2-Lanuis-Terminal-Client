# AI Agent / 贡献者短约定（Client）

完整说明见 [README.md](README.md) 与 [docs/](docs/README.md)。

- **禁止直推 `main`**；发版钉 Web tag → [docs/RELEASE.md](docs/RELEASE.md)
- 平台：Windows + Android；`web/` 为 submodule（见 `.gitmodules`）
- Rust：`cfg(desktop)` / `cfg(mobile)`；前端平台检测在 Web `platform.ts`
- 桌面 / Android 细节：[docs/DESKTOP.md](docs/DESKTOP.md)、[docs/ANDROID.md](docs/ANDROID.md)
- 合成完成用 notification 插件；Android CI 以 Linux 为准
