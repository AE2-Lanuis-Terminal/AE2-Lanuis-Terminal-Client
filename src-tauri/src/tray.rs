//! 系统托盘：任务栏旁图标 + 右键菜单（显示主窗口 / 设置 / 退出）。
//!
//! 约束与坑：
//! - 左键单击抬起：显示主窗；左键不弹菜单（`show_menu_on_left_click(false)`），
//!   避免与「单击显示」抢事件。
//! - 菜单「设置」在同步托盘回调里打开：必须 `async_runtime::spawn`，
//!   否则 Windows 上 WebView2 建窗死锁。
//! - 「退出」不直接 `app.exit`：向前端发 `tray-request-exit`，由 UI 确认后
//!   再调 `exit_app`，与主窗关闭确认策略一致。
//! - 文案可由前端 `sync_tray_menu_labels` 按 i18n 刷新，避免硬编码语言。

use serde::Deserialize;
// 托盘：Image / Menu / TrayIcon*；Emitter 给主窗发退出确认事件
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};

// 复用通用显隐 API；主窗 label 与 lib 关闭逻辑一致
use crate::window_api::{self, MAIN_LABEL};

/// 托盘图标实例 id，后续 `tray_by_id` 换菜单/tooltip 时用。
const TRAY_ID: &str = "main-tray";
/// 「显示主窗口」菜单项 id（重建菜单时须保持同一 id）。
const MENU_SHOW: &str = "ae2_tray_show";
/// 「设置」菜单项 id。
const MENU_SETTINGS: &str = "ae2_tray_settings";
/// 「退出」菜单项 id。
const MENU_EXIT: &str = "ae2_tray_exit";

// 菜单 id 在 create_tray 与 sync_tray_menu_labels 间必须稳定，否则事件匹配失败。

/// 从打包资源加载 32×32 PNG 作为托盘图标。
///
/// 用 `include_bytes!` 避免运行时找路径失败；尺寸过小在高 DPI 上可能糊，
/// 但与仓库 `icons/32x32.png` 约定一致。
fn tray_icon() -> tauri::Result<Image<'static>> {
    // 编译期嵌入，发布包无需额外拷贝图标文件
    const PNG: &[u8] = include_bytes!("../icons/32x32.png");
    // 解析为 Tauri Image；格式错误会在启动时报错
    Image::from_bytes(PNG)
}

/// 向主窗 Webview 发无载荷事件；主窗不存在时静默忽略。
///
/// 用于托盘「退出」等需前端参与确认的路径。
fn emit_to_main(app: &AppHandle, event: &str) {
    // 仅主窗需要处理；设置窗不订阅这些全局托盘事件
    if let Some(w) = app.get_webview_window(MAIN_LABEL) {
        // emit 失败通常因窗正在销毁，忽略即可
        let _ = w.emit(event, ());
    }
}

/// 创建托盘图标与右键菜单，并注册点击/菜单回调。
///
/// 应在 `Builder::setup` 中调用一次；失败则整个应用启动失败（无托盘则关闭到托盘不可用）。
pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    // 初始中文文案；前端就绪后会用 sync_tray_menu_labels 覆盖为当前语言
    let show = MenuItem::with_id(app, MENU_SHOW, "显示主窗口", true, None::<&str>)?;
    // 「设置」打开独立无边框窗，不替换主窗内容
    let settings = MenuItem::with_id(app, MENU_SETTINGS, "设置", true, None::<&str>)?;
    // 分隔线：把危险操作「退出」与常用项分开
    let sep = PredefinedMenuItem::separator(app)?;
    // 退出走前端确认，此处仅占位文案
    let exit = MenuItem::with_id(app, MENU_EXIT, "退出", true, None::<&str>)?;
    // 顺序：显示 → 设置 → 分隔 → 退出
    let menu = Menu::with_items(app, &[&show, &settings, &sep, &exit])?;

    // with_id 便于日后换菜单；无 id 则 sync 找不到实例
    TrayIconBuilder::with_id(TRAY_ID)
        // 嵌入的 32×32 图标
        .icon(tray_icon()?)
        // 悬停提示；后续可被 sync_tray_menu_labels 替换
        .tooltip("AE2 Lanuis Web Terminal")
        // 绑定刚建好的菜单
        .menu(&menu)
        // 左键只负责显示主窗，菜单仅右键，避免双行为冲突
        .show_menu_on_left_click(false)
        // 按菜单项 id 分发；id 与常量必须一致
        .on_menu_event(|app, event| match event.id.as_ref() {
            // 显示并聚焦主窗（可能从托盘隐藏态恢复）
            MENU_SHOW => {
                window_api::show_window(app, MAIN_LABEL);
            }
            // 打开设置：托盘回调是同步的，必须 spawn，否则 Windows 建窗会死锁
            MENU_SETTINGS => {
                // clone AppHandle：spawn 的 future 需 'static
                let app = app.clone();
                // 异步上下文中再调 open_settings_window（内部会 build Webview）
                tauri::async_runtime::spawn(async move {
                    // 打开失败时仅丢弃：托盘路径不宜弹原生错误框
                    let _ = crate::settings_window::open_settings_window(app).await;
                });
            }
            // 不直接 exit：交给前端确认（可能有未保存状态或「记住选择」）
            MENU_EXIT => {
                emit_to_main(app, "tray-request-exit");
            }
            // 未知 id：忽略，避免未来扩展菜单项时 panic
            _ => {}
        })
        // 图标本体点击（非菜单项）
        .on_tray_icon_event(|tray, event| {
            // 仅处理左键抬起，避免按下+抬起各触发一次导致闪烁
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                // 与菜单「显示」同一路径
                window_api::show_window(tray.app_handle(), MAIN_LABEL);
            }
        })
        // 注册到 App；失败向上返回 setup 错误
        .build(app)?;

    // 托盘已就绪；菜单文案可稍后由前端 i18n 覆盖
    Ok(())
}

/// 前端 invoke：显示主窗口（例如关闭确认对话框选「取消」后恢复）。
#[tauri::command]
pub fn show_main_window_cmd(app: AppHandle) -> Result<(), String> {
    // 不创建新窗；主窗应在启动时已存在
    window_api::show_window(&app, MAIN_LABEL);
    // 无窗口时 show_window 静默；仍返回 Ok 以免前端误报
    Ok(())
}

/// 前端 invoke：隐藏主窗口（「关闭到托盘」策略）。
///
/// 不销毁 Webview，进程与登录态保留；用户可从托盘再打开。
#[tauri::command]
pub fn hide_main_window_cmd(app: AppHandle) -> Result<(), String> {
    // hide 而非 close：保留前端 Pinia/会话
    window_api::hide_window(&app, MAIN_LABEL);
    // 隐藏成功与否对前端都返回 Ok（窗不存在亦无害）
    Ok(())
}

/// 前端确认后真正退出进程（托盘退出或关闭策略为「直接退出」）。
#[tauri::command]
pub fn exit_app(app: AppHandle) {
    // 退出码 0：正常关闭；托盘图标随进程结束由系统清理
    app.exit(0);
}

/// 前端推送的托盘菜单文案（camelCase，与 vue-i18n 键值对接）。
#[derive(Debug, Deserialize)]
// 与前端 payload 字段名对齐（show/settings/exit/tooltip）
#[serde(rename_all = "camelCase")]
pub struct TrayMenuLabelsPayload {
    /// 「显示主窗口」项文本。
    pub show: String,
    /// 「设置」项文本。
    pub settings: String,
    /// 「退出」项文本。
    pub exit: String,
    /// 托盘悬停 tooltip。
    pub tooltip: String,
}

/// 按当前语言重建托盘菜单并更新 tooltip。
///
/// Tauri 菜单项文本不便就地改写，故用相同 id 重建整棵菜单再 `set_menu`。
/// 须在托盘 `create_tray` 成功之后调用。
#[tauri::command]
pub fn sync_tray_menu_labels(app: AppHandle, payload: TrayMenuLabelsPayload) -> Result<(), String> {
    // 用前端传入文案重建三项 + 分隔线，id 与 create_tray 保持一致
    let show = MenuItem::with_id(&app, MENU_SHOW, &payload.show, true, None::<&str>)
        .map_err(|e| e.to_string())?;
    // settings / exit 同样映射错误为 String 给前端
    let settings = MenuItem::with_id(&app, MENU_SETTINGS, &payload.settings, true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let sep = PredefinedMenuItem::separator(&app).map_err(|e| e.to_string())?;
    let exit = MenuItem::with_id(&app, MENU_EXIT, &payload.exit, true, None::<&str>)
        .map_err(|e| e.to_string())?;
    // 菜单树结构须与 create_tray 相同，否则用户看到顺序跳变
    let menu = Menu::with_items(&app, &[&show, &settings, &sep, &exit]).map_err(|e| e.to_string())?;

    // 启动早期若误调，给出明确错误而非 panic
    let tray = app
        .tray_by_id(TRAY_ID)
        .ok_or_else(|| "tray icon not initialized".to_string())?;
    // 替换右键菜单；旧 Menu 由 set_menu 接管生命周期
    tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
    // tooltip 与菜单语言同步
    tray.set_tooltip(Some(payload.tooltip))
        .map_err(|e| e.to_string())?;
    Ok(())
}
