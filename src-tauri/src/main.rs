//! Client 可执行入口（Windows 桌面）。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    ae2_lanuis_client_lib::run();
}
