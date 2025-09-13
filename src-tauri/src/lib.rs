use tauri::Manager;

mod app;
pub use self::app::*;
mod command;
pub use self::command::*;
mod gamelist;
pub use self::gamelist::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let appdata = App::default();
	let inner = appdata.clone();

	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::TRACE) // Set the maximum level to log
		.init();

	tauri::async_runtime::spawn(
		async move { inner.event_loop().await },
	);

	tauri::Builder::default()
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
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
