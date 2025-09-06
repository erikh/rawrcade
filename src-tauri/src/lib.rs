use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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
	pub description: Option<String>,
	pub photo: Option<String>,
	pub gamelist: Vec<Game>,
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
			description: None,
			photo: None,
			gamelist: vec![Game::default()],
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct App {
	pub all_systems: Arc<Mutex<Vec<System>>>,
	pub orientation: Orientation,
}

impl App {
	pub async fn event_loop(&self) {
		while let Ok(event) = self.next_event().await {
			match event.typ {
				EventType::Input(e) => {
					tracing::trace!("input event: {:?}", e)
				}
			}
		}
	}

	pub async fn next_event(&self) -> Result<Event> {
		Ok(Event {
			typ: EventType::Input(InputEvent::Up),
		})
	}
}

#[tauri::command]
fn all_systems() -> Vec<System> {
	vec![System::default()]
}

#[tauri::command]
fn current_orientation() -> Option<Orientation> {
	Some(Orientation::default())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let app = App::default();
	let inner = app.clone();

	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::INFO) // Set the maximum level to log
		.init();

	tauri::async_runtime::spawn(
		async move { inner.event_loop().await },
	);

	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			all_systems,
			current_orientation
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
