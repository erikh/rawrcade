use std::sync::OnceLock;
use tauri::{AppHandle, Manager};

mod app;
mod command;
mod config;
mod gamelist;
mod gamepad;
mod systems;

pub use self::app::*;
pub use self::command::*;
pub use self::config::*;
pub use self::gamelist::*;
pub(crate) use self::gamepad::*;
pub use self::systems::*;

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
	let appdata = App::new(None).expect("could not initialize application");
	let inner = appdata.clone();
	let config = appdata.config.clone();

	tracing_subscriber::fmt()
		.with_max_level(Into::<tracing::Level>::into(
			appdata.config.lock().await.log_level.clone(),
		))
		.init();

	let sender = appdata.input_send.clone();

	tauri::async_runtime::spawn(async move { handle_gamepad_input(sender).await });

	tauri::async_runtime::spawn(async move { inner.event_loop().await });

	// initial fullscreen
	// FIXME: should probably replace this code once program settings have arrived
	tauri::async_runtime::spawn(async move {
		loop {
			if let Some(app_handle) = APP_HANDLE.get() {
				if let Some(window) = app_handle.get_window("main") {
					window
						.set_fullscreen(config.lock().await.start_fullscreen)
						.unwrap();
					return;
				}
			}
		}
	});

	let context = tauri::generate_context!();

	let builder = tauri::Builder::default()
		.setup(|app| {
			app.manage(appdata);
			Ok(())
		})
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			all_systems,
			current_orientation,
			current_asset,
			current_text,
			menu,
			setting_types,
			settings_menu,
			setting_values,
		])
		.build(context)
		.unwrap();

	APP_HANDLE.set(builder.app_handle().clone()).unwrap();
	builder.run(|_, _| {});
}
