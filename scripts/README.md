# 将兄弟 Web 仓链接为 ../web（Windows Junction）

Client 构建假定 `../web` 存在（`npm run build` → `npm run --prefix ../web build`）。

若本机 Web 仓名为 `AE2-Lanuis-Terminal-Web`，运行：

```powershell
.\scripts\link-web.ps1
# 或指定路径
.\scripts\link-web.ps1 -WebPath "E:\GIT\AE2-Lanuis-Terminal-Web"
```

CI 不使用本脚本，而是 `actions/checkout` 到 `../web`。
