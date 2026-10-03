// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg(target_os = "linux")]
use webkit2gtk::{
    LoadEvent, PermissionRequestExt, SettingsExt, UserContentInjectedFrames,
    UserContentManagerExt, UserScript, UserScriptInjectionTime, WebViewExt,
};

fn main() {
    // 1. Arka plan sesinin ve medya oynatımının pencere gizlendiğinde kesilmesini önle
    std::env::set_var("WEBKIT_DISABLE_BACKGROUND_TAB_THROTTLING", "1");
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "0");

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Ana pencereyi al
            let window = app.get_webview_window("main").expect("Ana pencere bulunamadı");

            // Linux WebKitGTK donanım izinleri, Chrome kimliği ve WebRTC yapılandırması
            #[cfg(target_os = "linux")]
            {
                let _ = window.with_webview(|webview| {
                    let wk_webview = webview.inner();
                    if let Some(settings) = wk_webview.settings() {
                        settings.set_user_agent(Some(
                            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
                        ));
                        settings.set_enable_webrtc(true);
                        settings.set_enable_media_stream(true);
                        settings.set_enable_mediasource(true);
                        settings.set_enable_developer_extras(true);
                    }

                    // Google OAuth ve Webview engellerini aşmak için başlangıç UserScript'i
                    if let Some(content_manager) = wk_webview.user_content_manager() {
                        let script = UserScript::new(
                            r#"
                            try {
                                Object.defineProperty(navigator, 'webdriver', {get: () => undefined});
                                if (!window.chrome) {
                                    window.chrome = { runtime: {} };
                                }
                            } catch(e) {}
                            "#,
                            UserContentInjectedFrames::AllFrames,
                            UserScriptInjectionTime::Start,
                            &[],
                            &[],
                        );
                        content_manager.add_script(&script);
                    }

                    // Kamera, mikrofon ve ekran paylaşımı isteklerine izin ver
                    wk_webview.connect_permission_request(|_view, request| {
                        request.allow();
                        true
                    });

                    // Sayfa yönlendirme koruması: Tanıtım sayfasına düşerse doğrudan calls ekranına yönlendir
                    wk_webview.connect_load_changed(|v, event| {
                        if event == LoadEvent::Committed {
                            if let Some(uri) = v.uri() {
                                if uri.contains("workspace.google.com") || uri.ends_with("/about") {
                                    v.load_uri("https://meet.google.com/calls");
                                }
                            }
                        }
                    });
                });
            }

            // Kapatma ('X') butonuna basıldığında pencereyi gizle, toplantıyı sonlandırma
            let win_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = win_clone.hide();
                }
            });

            // Sistem Tepsisi (Tray) Menüsü
            let toggle_item = MenuItem::with_id(app_handle, "toggle", "Toplantıyı Aç / Gizle", true, None::<&str>)?;
            let calls_item = MenuItem::with_id(app_handle, "calls", "Arama Yap ya da Katıl (Ana Sayfa)", true, None::<&str>)?;
            let new_meet_item = MenuItem::with_id(app_handle, "new_meet", "Yeni Toplantı Başlat (meet.new)", true, None::<&str>)?;
            let signin_item = MenuItem::with_id(app_handle, "signin", "Google Hesabıyla Oturum Aç", true, None::<&str>)?;
            let reload_item = MenuItem::with_id(app_handle, "reload", "Sayfayı Yenile", true, None::<&str>)?;
            let exit_item = MenuItem::with_id(app_handle, "exit", "Çıkış", true, None::<&str>)?;

            let tray_menu = Menu::with_items(
                app_handle,
                &[
                    &toggle_item,
                    &calls_item,
                    &new_meet_item,
                    &signin_item,
                    &reload_item,
                    &exit_item,
                ],
            )?;

            // İkonu yükle
            let icon_bytes = include_bytes!("../icons/32x32.png");
            let icon_image = image::load_from_memory(icon_bytes)
                .expect("İkon yüklenemedi")
                .to_rgba8();
            let (width, height) = icon_image.dimensions();
            let rgba_data = icon_image.into_raw();
            let tray_icon_image = tauri::image::Image::new_owned(rgba_data, width, height);

            // Sistem tepsisini oluştur
            let _tray = TrayIconBuilder::new()
                .icon(tray_icon_image)
                .tooltip("Google Meet v0.1.1")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "toggle" => {
                            if let Some(win) = app.get_webview_window("main") {
                                if win.is_visible().unwrap_or(false) {
                                    let _ = win.hide();
                                } else {
                                    let _ = win.show();
                                    let _ = win.set_focus();
                                }
                            }
                        }
                        "calls" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                                let _ = win.eval("window.location.href = 'https://meet.google.com/calls';");
                            }
                        }
                        "new_meet" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                                let _ = win.eval("window.location.href = 'https://meet.google.com/new';");
                            }
                        }
                        "signin" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                                let _ = win.eval("window.location.href = 'https://accounts.google.com/ServiceLogin?continue=https://meet.google.com/calls';");
                            }
                        }
                        "reload" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.eval("window.location.reload();");
                            }
                        }
                        "exit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                })
                .build(app_handle)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Google Meet uygulaması başlatılamadı");
}
