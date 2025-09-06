use serde::{Deserialize, Serialize};

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
pub struct Event {
	#[serde(rename = "type")]
	pub typ: EventType,
}

#[tauri::command]
fn next_event() -> Event {
	Event {
		typ: EventType::Input(InputEvent::Up),
	}
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

#[tauri::command]
fn change_systems(_orientation: Orientation) {}

#[tauri::command]
fn current_system() -> System {
	System::default()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			next_event,
			change_systems,
			current_system
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
