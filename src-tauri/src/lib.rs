use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputEvent {
	Up,
	Down,
	Left,
	Right,
	Ok,
	Cancel,
	Delete,
	Menu,
	Quit,
	PageUp,
	PageDown,
	First,
	Last,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "args", rename_all = "snake_case")]
pub enum EventType {
	Input(InputEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Event {
	#[serde(rename = "type")]
	pub typ: EventType,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Orientation {
	pub system_index: usize,
	pub gamelist_index: usize,
	pub menu_active: bool,
	pub menu_index: Option<usize>,
	pub menu_item_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct System {
	pub name: String,
	pub tag: String,
	pub description: Option<String>,
	pub photo: Option<String>,
	pub gamelist: Vec<Game>,
}

impl System {
	pub fn default_template(
		name: &str, tag: &str, gamelist: Vec<&str>,
	) -> Self {
		Self {
			name: name.to_string(),
			tag: tag.to_string(),
			gamelist: gamelist
				.iter()
				.map(|name| Game {
					name: name.to_string(),
				})
				.collect(),
			..Default::default()
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Game {
	pub name: String,
}

impl Default for Game {
	fn default() -> Self {
		Self {
			name: "Default Game".into(),
		}
	}
}

impl Default for System {
	fn default() -> Self {
		Self {
			name: "Default System".into(),
			tag: "default".into(),
			description: None,
			photo: None,
			gamelist: vec![Game::default()],
		}
	}
}

#[derive(Debug, Clone)]
pub struct App {
	pub all_systems: Arc<Mutex<Vec<System>>>,
	pub orientation: Arc<Mutex<Orientation>>,
}

impl Default for App {
	fn default() -> Self {
		Self {
			all_systems: Arc::new(Mutex::new(vec![
				System::default_template(
					"Nintendo",
					"nes",
					vec![
						"Super Mario Brothers",
						"Deadly Towers",
						"Karnov",
					],
				),
				System::default_template(
					"Super Nintendo",
					"snes",
					vec![
						"Bubsy",
						"Street Fighter 2",
						"Legend of Zelda: A Link to the Past",
					],
				),
				System::default_template(
					"Sony Playstation",
					"psx",
					vec![
						"Metal Gear Solid",
						"Castlevania: Symphony of the Night",
						"Banjo Kazooie",
					],
				),
			])),
			orientation: Arc::new(Mutex::new(Orientation::default())),
		}
	}
}

impl App {
	pub async fn event_loop(&self) {
		while let Ok(event) = self.next_event().await {
			match event.typ {
				EventType::Input(e) => {
					tracing::trace!("input event: {:?}", e);
					match e {
						InputEvent::Right => {
							let len =
								self.all_systems.lock().await.len() - 1;
							let mut lock =
								self.orientation.lock().await;
							if lock.system_index >= len {
								lock.system_index = 0;
							} else {
								lock.system_index += 1;
							}

							lock.gamelist_index = 0;
						}
						InputEvent::Left => {
							let len =
								self.all_systems.lock().await.len() - 1;
							let mut lock =
								self.orientation.lock().await;
							if lock.system_index == 0 {
								lock.system_index = len;
							} else {
								lock.system_index -= 1;
							}

							lock.gamelist_index = 0;
						}
						InputEvent::Up => {
							let mut lock =
								self.orientation.lock().await;
							let len = self.all_systems.lock().await
								[lock.system_index]
								.gamelist
								.len() - 1;

							if lock.gamelist_index == 0 {
								lock.gamelist_index = len;
							} else {
								lock.gamelist_index -= 1;
							}
						}
						InputEvent::Down => {
							let mut lock =
								self.orientation.lock().await;
							let len = self.all_systems.lock().await
								[lock.system_index]
								.gamelist
								.len() - 1;

							if lock.gamelist_index == len {
								lock.gamelist_index = 0;
							} else {
								lock.gamelist_index += 1;
							}
						}
						_ => {}
					}
				}
			}
		}
	}

	pub async fn next_event(&self) -> Result<Event> {
		tokio::time::sleep(std::time::Duration::from_secs(5)).await;

		let input = match rand::random::<u8>() % 4 {
			2 => InputEvent::Right,
			1 => InputEvent::Left,
			0 => InputEvent::Up,
			_ => InputEvent::Down,
		};

		Ok(Event {
			typ: EventType::Input(input),
		})
	}
}

#[tauri::command]
async fn all_systems(
	state: State<'_, App>,
) -> std::result::Result<Vec<System>, ()> {
	Ok(state.all_systems.clone().lock_owned().await.clone())
}

#[tauri::command]
async fn current_orientation(
	state: State<'_, App>,
) -> std::result::Result<Orientation, ()> {
	Ok(state.orientation.clone().lock_owned().await.clone())
}

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
			current_orientation
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
