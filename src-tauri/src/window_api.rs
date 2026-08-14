//! 通用窗口 API：按 label 打开 / 聚焦 / 关闭 / 显示 / 隐藏 Webview 窗口。
//!
//! 主窗与设置窗共用本模块，避免两处各自拼 `WebviewWindowBuilder`。
//!
//! **Windows / WebView2 约束**：
//! - `WebviewWindowBuilder::build` 在同步 `#[tauri::command]` 或同步事件回调里会与
//!   WebView2 消息循环死锁，表现为白屏、无响应、托盘菜单卡住。
//! - 调用方必须在 `async` 命令，或 `tauri::async_runtime::spawn` 里调用
//!   `open_or_focus_window`；`focus` / `show` / `hide` / `close` 相对安全。
//!
//! **无边框**：新建窗统一 `decorations(false)`，标题栏由前端自定义
//!（拖拽用 `bindTauriWindowDragRegion`，勿只靠 `data-tauri-drag-region`）。

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// 主窗口在 Tauri 配置里的 label，托盘与关闭拦截都依赖此常量。
pub const MAIN_LABEL: &str = "main";

/// 打开新 Webview 窗口所需的静态参数（label / 路由 / 尺寸）。
///
/// 使用 `'static` 字符串便于在模块内用 `const` 声明设置窗配置，
/// 避免每次打开都在堆上拼路径。
#[derive(Debug, Clone, Copy)]
pub struct WindowOpenOptions {
    /// 窗口唯一标识；已存在则聚焦，不重复建窗。
    pub label: &'static str,
    /// 相对前端入口，例如 `index.html#/settings`（hash 路由进设置页）。
    pub path: &'static str,
    /// 原生窗口标题（无边框时用户几乎看不到，仍写入便于任务管理器识别）。
    pub title: &'static str,
    /// 初始客户区宽度（逻辑像素）。
    pub width: f64,
    /// 初始客户区高度（逻辑像素）。
    pub height: f64,
    /// 最小宽度，防止缩到自定义标题栏无法操作。
    pub min_width: f64,
    /// 最小高度，防止设置页表单被裁切。
    pub min_height: f64,
}

/// 若窗口已存在：还原最小化 → 显示 → 聚焦；返回是否找到该 label。
///
/// 用于「再次打开设置」或托盘「显示主窗口」：只激活，不新建进程级窗口。
pub fn focus_window(app: &AppHandle, label: &str) -> bool {
    // 已有实例：按序 unminimize / show / set_focus，忽略单步失败以免中断链
    if let Some(w) = app.get_webview_window(label) {
        // 任务栏最小化后仅 show 可能仍不可见，先 unminimize
        let _ = w.unminimize();
        // 确保从隐藏/托盘态回到可见
        let _ = w.show();
        // 抢焦点；失败时窗口仍可见，可忽略
        let _ = w.set_focus();
        true
    } else {
        // label 不存在：调用方应走 open_or_focus_window 建新窗
        false
    }
}

/// 显示并聚焦指定 label 的窗口；不存在时静默忽略。
///
/// 托盘左键、菜单「显示主窗口」走此路径，不负责创建。
pub fn show_window(app: &AppHandle, label: &str) {
    // 复用 focus_window：统一最小化恢复与聚焦顺序
    let _ = focus_window(app, label);
}

/// 隐藏窗口到不可见（常用于「关闭到托盘」）；不销毁 Webview。
///
/// 与 `close` 不同：隐藏后可再 `show`，会话与前端状态保留。
pub fn hide_window(app: &AppHandle, label: &str) {
    // 仅当窗口仍存活时 hide；已销毁则无需处理
    if let Some(w) = app.get_webview_window(label) {
        // hide 失败不向上抛：托盘路径应尽量不打扰用户
        let _ = w.hide();
    }
}

/// 关闭并销毁指定 label 的窗口；不存在则视为成功。
///
/// 设置窗用此 API；主窗关闭由前端确认后再决定 hide 或 exit。
pub fn close_window(app: &AppHandle, label: &str) -> Result<(), String> {
    // 有实例才 close；无实例返回 Ok，便于幂等调用
    if let Some(w) = app.get_webview_window(label) {
        // 映射为 String，方便直接作为 Tauri command 错误返回前端
        w.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 打开或聚焦窗口。调用方须保证处于 async 上下文（见模块文档）。
///
/// 流程：先 `focus_window`；失败则按 `WindowOpenOptions` 新建无边框可缩放窗并居中。
pub fn open_or_focus_window(app: &AppHandle, opts: WindowOpenOptions) -> Result<(), String> {
    // 已打开：只聚焦，避免重复 WebView2 实例与多余内存
    if focus_window(app, opts.label) {
        return Ok(());
    }

    // 新建：App URL 指向打包后的前端资源 + hash 路由
    // 注意：此 build 必须在 async 上下文中调用，否则 Windows 会死锁
    WebviewWindowBuilder::new(app, opts.label, WebviewUrl::App(opts.path.into()))
        // 标题供系统识别；实际标题栏由 Vue 绘制
        .title(opts.title)
        // 初始尺寸按业务页设计（设置页偏窄高）
        .inner_size(opts.width, opts.height)
        // 下限防止自定义标题栏按钮点不到
        .min_inner_size(opts.min_width, opts.min_height)
        // 允许用户拖拽改大小；最小尺寸仍生效
        .resizable(true)
        // 无边框：与主窗一致，统一自定义标题栏体验
        .decorations(false)
        // 相对主显示器居中，避免设置窗飞出可视区
        .center()
        // build 失败转为可读错误给前端 toast
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}
