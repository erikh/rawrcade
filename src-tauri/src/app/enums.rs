use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigSettings {
	SwapConfirm,
	StartFullscreen,
	Theme,
	EnableKeyboard,
}

impl ConfigSettings {
	pub fn type_for(&self) -> String {
		match self {
			Self::SwapConfirm => "boolean",
			Self::StartFullscreen => "boolean",
			Self::Theme => "string",
			Self::EnableKeyboard => "boolean",
		}
		.to_string()
	}
}

impl From<usize> for ConfigSettings {
	fn from(value: usize) -> Self {
		match value {
			0 => Self::SwapConfirm,
			1 => Self::StartFullscreen,
			2 => Self::Theme,
			3 => Self::EnableKeyboard,
			_ => panic!("Invalid menu item"),
		}
	}
}

impl std::fmt::Display for ConfigSettings {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			ConfigSettings::SwapConfirm => "Japanese-style Input",
			ConfigSettings::StartFullscreen => "Start in Fullscreen",
			ConfigSettings::Theme => "Set Theme",
			ConfigSettings::EnableKeyboard => "Enable Keyboard",
		})
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuItems {
	Settings,
	Fullscreen,
	Exit,
	Reboot,
	Shutdown,
}

impl From<usize> for MenuItems {
	fn from(value: usize) -> Self {
		match value {
			0 => Self::Settings,
			1 => Self::Fullscreen,
			2 => Self::Exit,
			3 => Self::Reboot,
			4 => Self::Shutdown,
			_ => panic!("Invalid menu item"),
		}
	}
}

impl std::fmt::Display for MenuItems {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			MenuItems::Settings => "Settings",
			MenuItems::Fullscreen => "Toggle Fullscreen Window",
			MenuItems::Exit => "Exit RAWRcade",
			MenuItems::Reboot => "Reboot System",
			MenuItems::Shutdown => "Shutdown System",
		})
	}
}

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
