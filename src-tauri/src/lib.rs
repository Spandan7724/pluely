// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod activate;
mod api;
mod shortcuts;
mod window;
mod db;
mod screen_protection;
use base64::Engine;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use tauri_plugin_http;
#[cfg(target_os = "macos")]
use tauri_plugin_macos_permissions;
use tauri_plugin_posthog::{init as posthog_init, PostHogConfig, PostHogOptions};
use xcap::Monitor;
use tauri::Manager;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

mod speaker;
use speaker::VadConfig;

#[derive(Default)]
pub struct AudioState {
    stream_task: Arc<Mutex<Option<JoinHandle<()>>>>,
    vad_config: Arc<Mutex<VadConfig>>,
    is_capturing: Arc<Mutex<bool>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn capture_to_base64() -> Result<String, String> {
    let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;
    let primary_monitor = monitors
        .into_iter()
        .find(|m| m.is_primary())
        .ok_or("No primary monitor found".to_string())?;

    let image = primary_monitor
        .capture_image()
        .map_err(|e| format!("Failed to capture image: {}", e))?;
    let mut png_buffer = Vec::new();
    PngEncoder::new(&mut png_buffer)
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            ColorType::Rgba8.into(),
        )
        .map_err(|e| format!("Failed to encode to PNG: {}", e))?;
    let base64_str = base64::engine::general_purpose::STANDARD.encode(png_buffer);

    Ok(base64_str)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Get PostHog API key
    let posthog_api_key = option_env!("POSTHOG_API_KEY")
        .unwrap_or("")
        .to_string();
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:pluely.db", db::migrations())
                .build(),
        )
        .manage(AudioState::default())
        .manage(shortcuts::WindowVisibility {
            is_hidden: Mutex::new(false),
        })
        .manage(shortcuts::RegisteredShortcuts::default())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_keychain::init())
        .plugin(tauri_plugin_shell::init()) // Add shell plugin
        .plugin(posthog_init(PostHogConfig {
            api_key: posthog_api_key,
            options: Some(PostHogOptions {
                // disable session recording
                disable_session_recording: Some(true),
                // disable pageview
                capture_pageview: Some(false),
                // disable pageleave
                capture_pageleave: Some(false),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .plugin(tauri_plugin_machine_uid::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_version,
            window::set_window_height,
            capture_to_base64,
            shortcuts::check_shortcuts_registered,
            shortcuts::get_registered_shortcuts,
            shortcuts::update_shortcuts,
            shortcuts::validate_shortcut_key,
            shortcuts::set_app_icon_visibility,
            shortcuts::set_always_on_top,
            activate::activate_license_api,
            activate::deactivate_license_api,
            activate::validate_license_api,
            activate::mask_license_key_cmd,
            activate::get_checkout_url,
            activate::secure_storage_save,
            activate::secure_storage_get,
            activate::secure_storage_remove,
            api::transcribe_audio,
            api::chat_stream,
            api::fetch_models,
            api::create_system_prompt,
            api::check_license_status,
            speaker::start_system_audio_capture,
            speaker::stop_system_audio_capture,
            speaker::manual_stop_continuous,
            speaker::check_system_audio_access,
            speaker::request_system_audio_access,
            speaker::get_vad_config,
            speaker::update_vad_config,
            speaker::get_capture_status,
            speaker::get_audio_sample_rate,
            speaker::list_system_audio_devices,
            speaker::get_default_audio_device,
            screen_protection::toggle_screen_protection
        ])
        .setup(|app| {
            // Setup main window positioning
            if let Err(error) = window::setup_main_window(app) {
                eprintln!("Failed to setup main window: {}", error);
            }

            // Initialize global shortcut plugin with centralized handler
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};
                        
                        if event.state() == ShortcutState::Pressed {
                            // Get registered shortcuts and find matching action
                            let state = app.state::<shortcuts::RegisteredShortcuts>();
                            let registered = match state.shortcuts.lock() {
                                Ok(guard) => guard,
                                Err(poisoned) => {
                                    eprintln!("Mutex poisoned in handler, recovering...");
                                    poisoned.into_inner()
                                }
                            };
                            
                            // Find which action this shortcut maps to
                            for (action_id, shortcut_str) in registered.iter() {
                                if let Ok(s) = shortcut_str.parse::<Shortcut>() {
                                    if &s == shortcut {
                                        eprintln!("Shortcut triggered: {} ({})", action_id, shortcut_str);
                                        shortcuts::handle_shortcut_action(&app, action_id);
                                        break;
                                    }
                                }
                            }
                        }
                    })
                    .build(),
            ).expect("Failed to initialize global shortcut plugin");
            if let Err(e) = shortcuts::setup_global_shortcuts(app.handle()) {
                eprintln!("Failed to setup global shortcuts: {}", e);
            }

            Ok(())
        });

    // Add macOS-specific permissions plugin
    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_plugin_macos_permissions::init());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
