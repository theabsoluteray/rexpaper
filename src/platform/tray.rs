use std::sync::Arc;
use windows::core::w;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW,
};
use windows::Win32::UI::WindowsAndMessaging::*;

const WM_TRAYICON: u32 = WM_USER + 101;
const ID_TRAY_OPEN: usize = 1001;
const ID_TRAY_STATIC: usize = 1002;
const ID_TRAY_LIVE: usize = 1003;
const ID_TRAY_SETTINGS: usize = 1004;
const ID_TRAY_QUIT: usize = 1005;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    Open,
    StaticPage,
    LivePage,
    SettingsPage,
    Quit,
}

pub fn setup_tray<F>(callback: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(TrayAction) + Send + Sync + 'static,
{
    let callback = Arc::new(callback);
    std::thread::Builder::new()
        .name("rexpaper-tray".to_string())
        .spawn(move || unsafe {
            let hinstance = GetModuleHandleW(None).unwrap_or_default();
            let class_name = w!("RexPaperTrayClass");

            let wnd_class = WNDCLASSW {
                lpfnWndProc: Some(tray_wnd_proc),
                hInstance: hinstance.into(),
                lpszClassName: class_name,
                ..Default::default()
            };

            let _ = RegisterClassW(&wnd_class);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                w!("RexPaperTrayWindow"),
                WS_OVERLAPPED,
                0,
                0,
                0,
                0,
                None,
                None,
                Some(hinstance.into()),
                None,
            )
            .unwrap_or_default();

            if hwnd.0.is_null() {
                return;
            }

            let mut nid = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: hwnd,
                uID: 1,
                uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
                uCallbackMessage: WM_TRAYICON,
                ..Default::default()
            };

            // Load application icon or default
            let icon = LoadIconW(Some(hinstance.into()), IDI_APPLICATION).unwrap_or_else(|_| {
                LoadIconW(None, IDI_APPLICATION).unwrap_or_default()
            });
            nid.hIcon = icon;

            // Set tooltip
            let tip = "RexPaper - Wallpaper Manager";
            for (i, c) in tip.encode_utf16().enumerate() {
                if i < nid.szTip.len() - 1 {
                    nid.szTip[i] = c;
                }
            }

            let _ = Shell_NotifyIconW(NIM_ADD, &nid);

            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).into() {
                if msg.message == WM_TRAYICON {
                    let event = msg.lParam.0 as u32;
                    if event == WM_LBUTTONUP || event == WM_LBUTTONDBLCLK {
                        callback(TrayAction::Open);
                    } else if event == WM_RBUTTONUP {
                        let mut pt = POINT::default();
                        let _ = GetCursorPos(&mut pt);
                        let _ = SetForegroundWindow(hwnd);

                        let menu = CreatePopupMenu().unwrap_or_default();
                        if !menu.0.is_null() {
                            let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_OPEN, w!("Open RexPaper"));
                            let _ = AppendMenuW(menu, MF_SEPARATOR, 0, None);
                            let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_STATIC, w!("Static Wallpapers"));
                            let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_LIVE, w!("Live Wallpapers"));
                            let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_SETTINGS, w!("Settings"));
                            let _ = AppendMenuW(menu, MF_SEPARATOR, 0, None);
                            let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_QUIT, w!("Quit RexPaper"));

                            let cmd = TrackPopupMenu(
                                menu,
                                TPM_RIGHTBUTTON | TPM_BOTTOMALIGN | TPM_RETURNCMD,
                                pt.x,
                                pt.y,
                                Some(0),
                                hwnd,
                                None,
                            );
                            let _ = DestroyMenu(menu);

                            match cmd.0 as usize {
                                ID_TRAY_OPEN => callback(TrayAction::Open),
                                ID_TRAY_STATIC => callback(TrayAction::StaticPage),
                                ID_TRAY_LIVE => callback(TrayAction::LivePage),
                                ID_TRAY_SETTINGS => callback(TrayAction::SettingsPage),
                                ID_TRAY_QUIT => callback(TrayAction::Quit),
                                _ => {}
                            }
                        }
                    }
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
        })?;

    Ok(())
}

unsafe extern "system" fn tray_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
