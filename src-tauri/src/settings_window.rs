//! 设置独立窗口：基于通用 `window_api`，与主窗分离。
//!
//! 动机：设置改关闭行为 / 主题时不应挡住主窗操作；
//! 独立 label 也便于主窗 `CloseRequested` 时直接放行设置窗关闭。
//!
//! 路由：`index.html#/settings`，与主窗共用同一前端包与会话存储。
//! 无边框 + 自定义标题栏，约束同主窗（见 `window_api` 模块说明）。

use tauri::AppHandle;

use crate::window_api::{self, WindowOpenOptions};

/// 设置窗 label；`lib` 关闭拦截与 `close_settings_window` 都依赖此常量。
pub const SETTINGS_LABEL: &str = "settings";

/// 设置窗静态几何与入口：窄高布局适配表单；min 尺寸保证标题栏可点。
const SETTINGS_WINDOW: WindowOpenOptions = WindowOpenOptions {
    // 与命令、关闭逻辑共用同一 label
    label: SETTINGS_LABEL,
    // hash 进设置页，不另起 Vite 入口
    path: "index.html#/settings",
    // 任务管理器 / 无障碍可读标题
    title: "AE2 Lanuis · Settings",
    // 初始宽：约一列表单
    width: 440.0,
    // 初始高：容纳关闭行为等选项
    height: 580.0,
    // 再窄则自定义标题栏易挤在一起
    min_width: 380.0,
    // 再矮则底部按钮被裁
    min_height: 480.0,
};

/// 打开或聚焦设置窗。
///
/// Windows 上必须用 `async` 命令：同步 invoke 里 `WebviewWindowBuilder::build`
/// 会与 WebView2 死锁，导致白屏/无响应。托盘菜单里也应 `spawn` 后再调本函数。
#[tauri::command]
pub async fn open_settings_window(app: AppHandle) -> Result<(), String> {
    // 委托通用 API：已存在则聚焦，否则异步安全地建无边框窗
    window_api::open_or_focus_window(&app, SETTINGS_WINDOW)
}

/// 关闭设置窗（销毁 Webview）；幂等，窗不存在也返回 Ok。
///
/// 供前端设置页「关闭」按钮调用；与主窗 hide-to-tray 策略无关。
#[tauri::command]
pub async fn close_settings_window(app: AppHandle) -> Result<(), String> {
    // 直接销毁；下次打开会重新加载 `#/settings`
    window_api::close_window(&app, SETTINGS_LABEL)
}
