use gilrs::{Axis, Button, Event as GamepadEvent, EventType as GamepadEventType, Gilrs};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc::Sender;

mod app;
mod command;
mod config;
mod gamelist;
mod systems;

pub use self::app::*;
pub use self::command::*;
pub use self::config::*;
pub use self::gamelist::*;
pub use self::systems::*;

pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

async fn handle_gamepad_input(sender: Sender<InputEvent>) {
	let mut gilrs = Gilrs::new().unwrap();
	let mut debounce: Option<Instant> = None;
	let mut latest_axis: Option<(gilrs::Axis, f32)> = None;

	'event_loop: loop {
		if let Some(GamepadEvent { id: _, event, .. }) = gilrs.next_event() {
			tracing::debug!("gamepad input event: {:?}", event);
			match event {
				GamepadEventType::AxisChanged(x, amp, ..) => {
					if let Some(inner) = debounce {
						if Instant::now() - inner < Duration::from_millis(200) {
							continue 'event_loop;
						} else {
							debounce = None;
						}
					}

					if let Some(inner) = latest_axis {
						if inner.0 == x {
							// NOTE: releasing the stick should not generate additional events. this
							// fixes that.
							if (inner.1 < 0.0 && amp > inner.1) || (inner.1 > 0.0 && amp < inner.1)
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
						Button::DPadDown => Some(InputEvent::Down),
						Button::DPadUp => Some(InputEvent::Up),
						Button::DPadLeft => Some(InputEvent::Left),
						Button::DPadRight => Some(InputEvent::Right),
						Button::Start => Some(InputEvent::Menu),
						Button::South => Some(InputEvent::Ok),
						Button::East => Some(InputEvent::Cancel),
						Button::LeftTrigger => Some(InputEvent::PageUp),
						Button::RightTrigger => Some(InputEvent::PageDown),
						_ => None,
					};

					if let Some(event) = event {
						let _ = sender.send(event).await;
					}
				}
				_ => {}
			}
		} else {
			tokio::time::sleep(Duration::from_millis(200)).await;
		}
	}
}

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
