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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
	Next,
	Previous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct System {
	pub name: String,
	pub description: Option<String>,
	pub photo: Option<String>,
}

impl Default for System {
	fn default() -> Self {
		Self {
			name: "Default System".into(),
			description: None,
			photo: None,
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct App {
	pub current_system: Arc<Mutex<System>>,
	pub all_systems: Arc<Mutex<Vec<System>>>,
}

impl App {
	pub async fn event_loop(&self) {
		while let Ok(event) = self.next_event().await {
			match event.typ {
				EventType::Input(e) => tracing::info!("{:?}", e),
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
fn current_system() -> System {
	System::default()
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
		.invoke_handler(tauri::generate_handler![current_system])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
