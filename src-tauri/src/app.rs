use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{Game, GameList};

#[derive(Debug, Clone)]
pub struct App {
	pub all_systems: Arc<Mutex<Vec<System>>>,
	pub orientation: Arc<Mutex<Orientation>>,
}

impl Default for App {
	fn default() -> Self {
		let gamelist = GameList::from_file("test-gamelist.xml".into())
			.unwrap()
			.game
			.iter()
			.map(|x| Into::<Game>::into(x.clone()))
			.collect::<Vec<Game>>();

		Self {
			all_systems: Arc::new(Mutex::new(vec![
				System::default_template(
					"Nintendo Entertainment System",
					"nes",
					gamelist.clone(),
				),
				System::default_template(
					"Super Nintendo Entertainment System",
					"snes",
					gamelist.clone(),
				),
				System::default_template(
					"Sony Playstation",
					"psx",
					gamelist.clone(),
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
		tokio::time::sleep(std::time::Duration::from_secs(60)).await;

		/*
			let input = match rand::random::<u8>() % 2 {
				2 => InputEvent::Right,
				1 => InputEvent::Down,
				0 => InputEvent::Up,
				_ => InputEvent::Left,
			};
		*/

		Ok(Event {
			typ: EventType::Input(InputEvent::Down),
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
		name: &str, tag: &str, gamelist: Vec<Game>,
	) -> Self {
		Self {
			name: name.to_string(),
			tag: tag.to_string(),
			gamelist,
			..Default::default()
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
