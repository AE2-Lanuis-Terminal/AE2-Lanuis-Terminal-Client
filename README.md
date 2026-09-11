# AE2 Lanuis Terminal — Client

在 **Windows** 或 **Android** 上使用 AE2 Lanuis 远程终端，无需在浏览器里收藏服务器地址。

前提：游戏服务器已安装 [AE2 Lanuis 模组](https://github.com/AE2-Lanuis-Terminal/AE2-Lanuis-Terminal)，且你已在游戏内绑定密码。

| 平台 | 说明 |
|------|------|
| Windows | 桌面安装包（托盘、无边框窗口） |
| Android | 手机 / 平板 APK |
| 许可 | [MIT](LICENSE) |

暂无 macOS / Linux 桌面版与 iOS。

## 安装

从本仓库的 **Releases** 下载对应平台安装包 / APK（通网发布后），或请服主提供构建好的文件。

- Windows：运行安装程序，按提示完成  
- Android：允许安装未知来源后安装 APK（仅 arm64 正式包时请确认设备架构）

## 登录

1. 打开应用  
2. 填写服务器 **IP（或域名）**、**端口**（默认多为 `8765`）、协议（一般 `http`）  
3. 输入游戏账号与你在游戏里设置的网页密码  
4. 连接后即可使用存储、合成、样板等功能  

密码在游戏内设置：持无线终端执行 `/ae2lanuis password <密码>`。  
网页地址与绑定状态：`/ae2lanuis status`。

## 小提示

- 服务器地址需要 **手动填写**，应用不会自动发现局域网主机  
- 开启「完成推送」并允许系统通知后，合成完成可收到通知（需保持应用可在后台联网）  
- 桌面端可最小化到托盘；具体关闭行为可在设置里选择  

装服、端口与图标等：**[主模组说明](https://github.com/AE2-Lanuis-Terminal/AE2-Lanuis-Terminal)**。

## 开发者

从源码构建、Android 本机打包坑等见 **[docs/README.md](docs/README.md)**。
