use tauri::Manager;

mod app;
pub use self::app::*;
mod command;
pub use self::command::*;
mod gamelist;
pub use self::gamelist::*;
use gilrs::{Button, Event as GamepadEvent, Gilrs};
use tokio::sync::mpsc::Sender;

async fn handle_gamepad_input(sender: Sender<InputEvent>) {
	let mut gilrs = Gilrs::new().unwrap();

	loop {
		while let Some(GamepadEvent {
			id: _,
			event,
			time: _,
			..
		}) = gilrs.next_event()
		{
			match event {
				gilrs::EventType::ButtonPressed(x, ..) => {
					eprintln!("{:?} pressed", x);
					let event = match x {
						Button::DPadDown => InputEvent::Down,
						_ => InputEvent::Down,
					};

					let _ = sender.send(event).await;
				}
				_ => {}
			}
		}
	}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let appdata = App::default();
	let inner = appdata.clone();

	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::DEBUG) // Set the maximum level to log
		.init();

	let sender = appdata.input_send.clone();

	tauri::async_runtime::spawn(async move {
		handle_gamepad_input(sender).await
	});

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
