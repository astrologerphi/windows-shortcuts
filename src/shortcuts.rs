use std::{sync::OnceLock, thread};

use crate::{
    alert,
    constants::APP_CONFIG,
    screen::{modes::CaptureMode, take_screenshot_for_windows},
    utils::{
        clipboard::clear_clipboard,
        explorer::kill_explorer,
        inputs::close_top_window,
        magic_packet::MagicPacket,
        monitors::{set_external_display, set_internal_display},
        night_light::disable_night_light,
    },
};

#[derive(Clone)]
pub struct Shortcut {
    pub id: Option<usize>,
    pub func: fn() -> (),
    pub is_left_click: bool,
    pub menu_name: Option<String>,
    pub web_req_url: Option<String>,
}

pub static SHORTCUTS: OnceLock<Vec<Shortcut>> = OnceLock::new();

pub fn build_shortcuts() {
    thread::spawn(|| {
        let _ = SHORTCUTS.get_or_init(|| {
            vec![
                Shortcut {
                    id: Some(8),
                    func: || {
                        let txt = APP_CONFIG.get().unwrap().screen_dir.to_owned();
                        alert!("{}", txt);
                    },
                    is_left_click: false,
                    menu_name: Some("Test".to_string()),
                    web_req_url: Some("/test_connection".to_string()),
                },
                Shortcut {
                    id: Some(19),
                    func: || {
                        let dir = APP_CONFIG.get().unwrap().screen_dir.to_owned();
                        let _ = take_screenshot_for_windows(&dir, CaptureMode::Primary);
                    },
                    is_left_click: false,
                    menu_name: Some("Capture Windows Screen".to_string()),
                    web_req_url: Some("/capture_windows_screen".to_string()),
                },
                Shortcut {
                    id: Some(10),
                    func: || {
                        let mac = APP_CONFIG.get().unwrap().tv_mac_addr;
                        thread::spawn(move || {
                            let magic_p = MagicPacket::new(&mac);
                            let res = magic_p.send();
                            if let Ok(_) = res {
                                set_external_display();
                                disable_night_light().unwrap();
                            }
                        });
                    },
                    is_left_click: false,
                    menu_name: Some("Switch to TV".to_string()),
                    web_req_url: Some("/switch_to_tv".to_string()),
                },
                Shortcut {
                    id: Some(11),
                    func: || {
                        thread::spawn(move || {
                            set_internal_display();
                        });
                    },
                    is_left_click: false,
                    menu_name: Some("Switch to Monitor".to_string()),
                    web_req_url: Some("/switch_to_monitor".to_string()),
                },
                Shortcut {
                    id: None,
                    func: || {
                        clear_clipboard();
                        kill_explorer();
                    },
                    is_left_click: true,
                    menu_name: None,
                    web_req_url: None,
                },
                Shortcut {
                    id: None,
                    func: || close_top_window(),
                    is_left_click: false,
                    menu_name: None,
                    web_req_url: Some("/close_top_window".to_string()),
                },
            ]
        });
    })
    .join()
    .unwrap();
}
