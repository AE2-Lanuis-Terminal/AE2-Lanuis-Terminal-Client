# 将已 clone 的 Web 仓链接为本仓 web/（Windows Junction）

正式流程：`git submodule update --init`（见 [.gitmodules](../.gitmodules)）。

本脚本仅在本机想复用兄弟目录的 Web 工作区时使用：

```powershell
.\scripts\link-web.ps1
# 或
.\scripts\link-web.ps1 -WebPath "E:\GIT\AE2-Lanuis-Terminal-Web"
```

CI 使用 `actions/checkout` 的 `submodules: recursive`，不跑本脚本。
