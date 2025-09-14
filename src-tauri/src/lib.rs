use gilrs::{
	Axis, Button, Event as GamepadEvent, EventType as GamepadEventType,
	Gilrs,
};
use std::time::{Duration, Instant};
use tauri::Manager;
use tokio::sync::mpsc::Sender;

mod app;
mod command;
mod gamelist;
mod systems;

pub use self::app::*;
pub use self::command::*;
pub use self::gamelist::*;
pub use self::systems::*;

async fn handle_gamepad_input(sender: Sender<InputEvent>) {
	let mut gilrs = Gilrs::new().unwrap();
	let mut debounce: Option<Instant> = None;
	let mut latest_axis: Option<(gilrs::Axis, f32)> = None;

	'event_loop: loop {
		while let Some(GamepadEvent { id: _, event, .. }) =
			gilrs.next_event()
		{
			tracing::debug!("gamepad input event: {:?}", event);
			match event {
				GamepadEventType::AxisChanged(x, amp, ..) => {
					if let Some(inner) = debounce {
						if Instant::now() - inner
							< Duration::from_millis(200)
						{
							continue 'event_loop;
						} else {
							debounce = None;
						}
					}

					if let Some(inner) = latest_axis {
						if inner.0 == x {
							// NOTE: releasing the stick should not generate additional events. this
							// fixes that.
							if (inner.1 < 0.0 && amp > inner.1)
								|| (inner.1 > 0.0 && amp < inner.1)
							{
								continue 'event_loop;
							}
						} else {
							latest_axis = None;
						}
					}

					let event = match x {
						Axis::LeftStickY => {
							if amp > 0.5 {
								Some(InputEvent::Up)
							} else if amp < -0.5 {
								Some(InputEvent::Down)
							} else {
								None
							}
						}
						Axis::LeftStickX => {
							if amp > 0.5 {
								Some(InputEvent::Right)
							} else if amp < -0.5 {
								Some(InputEvent::Left)
							} else {
								None
							}
						}
						_ => None,
					};

					if let Some(event) = event {
						latest_axis = Some((x, amp));
						debounce = Some(Instant::now());
						let _ = sender.send(event).await;
					}
				}
				GamepadEventType::ButtonPressed(x, ..) => {
					let event = match x {
						Button::DPadDown => InputEvent::Down,
						Button::DPadUp => InputEvent::Up,
						Button::DPadLeft => InputEvent::Left,
						Button::DPadRight => InputEvent::Right,
						Button::Start => InputEvent::Menu,
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
			menu,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
