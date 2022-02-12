#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use tauri::{CustomMenuItem, Icon, SystemTray, SystemTrayEvent, SystemTrayHandle, SystemTrayMenu};
use windows::Devices::{
    Bluetooth::{BluetoothConnectionStatus, BluetoothLEDevice},
    Enumeration::DeviceInformation,
};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new()
        .with_icon(Icon::Raw(std::fs::read("icons/disconnected.ico").unwrap()))
        .with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                if id.as_str() == "quit" {
                    app.exit(0);
                }
            }
        })
        .setup(|app| {
            let tray = app.tray_handle();
            spawn(move || watch_device(tray));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn watch_device(tray: SystemTrayHandle) {
    loop {
        if is_hhkb_connected() {
            tray.set_icon(Icon::Raw(std::fs::read("icons/connected.ico").unwrap()))
                .unwrap();
            sleep(Duration::from_secs(10));
        } else {
            tray.set_icon(Icon::Raw(std::fs::read("icons/disconnected.ico").unwrap()))
                .unwrap();
            sleep(Duration::from_millis(500));
        }
    }
}

fn is_hhkb_connected() -> bool {
    connected_btle_devices()
        .iter()
        .any(|x| x.Name().unwrap().to_string().starts_with("HHKB-Hybrid_"))
}

fn connected_btle_devices() -> Vec<DeviceInformation> {
    let filter = BluetoothLEDevice::GetDeviceSelectorFromConnectionStatus(
        BluetoothConnectionStatus::Connected,
    )
    .unwrap();
    let list = DeviceInformation::FindAllAsyncAqsFilter(filter)
        .unwrap()
        .get()
        .unwrap();
    (0..list.Size().unwrap())
        .map(|x| list.GetAt(x).unwrap())
        .collect::<Vec<_>>()
}
