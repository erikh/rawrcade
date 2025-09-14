use crate::{Game, GameList};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::{
	Mutex,
	mpsc::{Receiver, Sender, channel},
};

#[derive(Debug, Clone)]
pub struct App {
	pub all_systems: Arc<Mutex<Vec<System>>>,
	pub orientation: Arc<Mutex<Orientation>>,
	pub input_send: Sender<InputEvent>,

	input_recv: Arc<Mutex<Receiver<InputEvent>>>,
}

impl Default for App {
	fn default() -> Self {
		let gamelist = GameList::from_file("test-gamelist.xml".into())
			.unwrap()
			.game
			.iter()
			.map(|x| Into::<Game>::into(x.clone()))
			.collect::<Vec<Game>>();

		let (s, r) = channel(1000);

		Self {
			input_send: s,
			input_recv: Arc::new(Mutex::new(r)),
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
					tracing::debug!("input event: {:?}", e);
					match e {
						InputEvent::Menu => {
							let mut orientation =
								self.orientation.lock().await;
							orientation.menu_active =
								!orientation.menu_active;
							if orientation.menu_active {
								orientation.menu_index = Some(0);
							}
						}
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
		loop {
			let mut chan = self.input_recv.lock().await;
			let item = chan.try_recv();
			if let Ok(item) = item {
				tracing::debug!("received event {:?}", item);

				return Ok(Event {
					typ: EventType::Input(item),
				});
			} else {
				tokio::time::sleep(Duration::from_millis(50)).await;
				continue;
			}
		}
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
