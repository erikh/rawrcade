use crate::{APP_HANDLE, Config, DEFAULT_CONFIG_FILENAME, Game, GameList, SystemList};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
	path::PathBuf,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};
use tauri::Manager;
use tokio::sync::{
	Mutex,
	mpsc::{Receiver, Sender, channel},
};

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

enum MenuItems {
	Settings,
	Themes,
	Fullscreen,
	Exit,
	Reboot,
	Shutdown,
}

impl From<usize> for MenuItems {
	fn from(value: usize) -> Self {
		match value {
			0 => Self::Settings,
			1 => Self::Themes,
			2 => Self::Fullscreen,
			3 => Self::Exit,
			4 => Self::Reboot,
			5 => Self::Shutdown,
			_ => panic!("Invalid menu item"),
		}
	}
}

impl std::fmt::Display for MenuItems {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			MenuItems::Settings => "Settings",
			MenuItems::Themes => "Themes",
			MenuItems::Fullscreen => "Toggle Fullscreen Window",
			MenuItems::Exit => "Exit RAWRcade",
			MenuItems::Reboot => "Reboot System",
			MenuItems::Shutdown => "Shutdown System",
		})
	}
}

#[derive(Debug, Clone)]
pub struct App {
	pub config: Arc<Mutex<Config>>,
	pub all_systems: Arc<Mutex<SystemList>>,
	pub orientation: Arc<Mutex<Orientation>>,
	pub input_send: Sender<InputEvent>,

	config_filename: PathBuf,
	input_recv: Arc<Mutex<Receiver<InputEvent>>>,
	ignore_events: Arc<AtomicBool>,
}

impl Default for App {
	fn default() -> Self {
		let mut all_systems = SystemList::from_file("test-systems.xml".into()).unwrap();

		let gamelist = GameList::from_file("test-gamelist.xml".into())
			.unwrap()
			.game
			.iter()
			.map(|x| Into::<Game>::into(x.clone()))
			.collect::<Vec<Game>>();

		for x in &mut all_systems.system {
			x.gamelist = gamelist.clone()
		}

		let (s, r) = channel(1000);

		let path = dirs::config_dir().unwrap_or(dirs::home_dir().unwrap_or("/".into()));

		Self {
			config_filename: path.join(DEFAULT_CONFIG_FILENAME),
			config: Arc::new(Mutex::new(Config::default())),
			input_send: s,
			input_recv: Arc::new(Mutex::new(r)),
			all_systems: Arc::new(Mutex::new(all_systems)),
			orientation: Arc::new(Mutex::new(Orientation::default())),
			ignore_events: Default::default(),
		}
	}
}

impl App {
	pub fn new(config_filename: Option<&PathBuf>) -> Result<Self> {
		let mut this = Self::default();
		if let Ok(config) = Config::from_file(config_filename.unwrap_or(&this.config_filename)) {
			this.config = Arc::new(Mutex::new(config))
		}

		Ok(this)
	}

	pub fn menu(&self) -> Vec<String> {
		vec![
			MenuItems::Settings.to_string(),
			MenuItems::Themes.to_string(),
			MenuItems::Fullscreen.to_string(),
			MenuItems::Exit.to_string(),
			MenuItems::Reboot.to_string(),
			MenuItems::Shutdown.to_string(),
		]
	}

	pub fn settings_menu(&self) -> Vec<String> {
		vec![
			ConfigSettings::SwapConfirm.to_string(),
			ConfigSettings::StartFullscreen.to_string(),
			ConfigSettings::Theme.to_string(),
			ConfigSettings::EnableKeyboard.to_string(),
		]
	}

	pub async fn event_loop(&self) {
		while let Ok(event) = self.next_event().await {
			if self.ignore_events.load(Ordering::SeqCst) {
				continue;
			}

			match event.typ {
				EventType::Input(e) => {
					tracing::debug!("input event: {:?}", e);
					match e {
						InputEvent::Cancel => {
							let mut orientation = self.orientation.lock().await;

							if orientation.menu_active {
								if orientation.menu_item_index.is_some() {
									orientation.menu_item_index = None;
								} else {
									orientation.menu_active = false;
									orientation.menu_index = None;
									orientation.menu_item_index = None;
								}
							}
						}
						InputEvent::Ok => {
							let mut orientation = self.orientation.lock().await;

							if orientation.menu_active {
								if let Some(idx) = orientation.menu_index {
									match MenuItems::from(idx) {
										MenuItems::Settings => match orientation.menu_item_index {
											Some(inner_idx) => {
												let mut config = self.config.lock().await;
												match ConfigSettings::from(inner_idx) {
													ConfigSettings::Theme => {}
													ConfigSettings::EnableKeyboard => {
														config.enable_keyboard =
															!config.enable_keyboard
													}
													ConfigSettings::StartFullscreen => {
														config.start_fullscreen =
															!config.start_fullscreen
													}
													ConfigSettings::SwapConfirm => {
														config.swap_confirm = !config.swap_confirm
													}
												}
											}
											None => {
												tracing::debug!("initializing inner menu state");
												orientation.menu_item_index = Some(0);
											}
										},
										MenuItems::Reboot => {
											self.config
												.lock()
												.await
												.to_file(&self.config_filename)
												.unwrap_or_else(|_| {
													panic!(
														"could not write file: {}",
														self.config_filename.display()
													)
												});

											std::process::Command::new("reboot")
												.status()
												.expect("could not reboot");
										}
										MenuItems::Shutdown => {
											self.config
												.lock()
												.await
												.to_file(&self.config_filename)
												.unwrap_or_else(|_| {
													panic!(
														"could not write file: {}",
														self.config_filename.display()
													)
												});

											std::process::Command::new("poweroff")
												.status()
												.expect("could not poweroff");
										}
										MenuItems::Fullscreen => {
											if let Some(app_handle) = APP_HANDLE.get() {
												if let Some(window) = app_handle.get_window("main")
												{
													window
														.set_fullscreen(
															!window.is_fullscreen().expect(
																"could not toggle fullscreen",
															),
														)
														.expect("Could not unset fullscreen state");
												}
											}
										}
										MenuItems::Exit => {
											self.config
												.lock()
												.await
												.to_file(&self.config_filename)
												.unwrap_or_else(|_| {
													panic!(
														"could not write file: {}",
														self.config_filename.display()
													)
												});

											std::process::exit(0);
										}
										_ => {}
									}
								}
							} else {
								let system =
									&self.all_systems.lock().await.system[orientation.system_index];

								let mut is_fullscreen = false;

								if let Some(app_handle) = APP_HANDLE.get() {
									if let Some(window) = app_handle.get_window("main") {
										is_fullscreen = window
											.is_fullscreen()
											.expect("could not get fullscreen state");

										if is_fullscreen {
											window
												.set_fullscreen(false)
												.expect("Could not unset fullscreen state");
										}
									}
								}

								self.ignore_events.store(true, Ordering::SeqCst);

								let game = system.gamelist[orientation.gamelist_index].clone();

								let command = system.get_command(
									game.path.expect("Need a path to the rom in gamelist.xml"),
								);

								let args = vec!["-c", &command];
								let mut child = std::process::Command::new("/bin/sh")
									.args(args)
									.spawn()
									// FIXME: probably should do something better here
									.expect("Could not boot emulator command");

								let s = self.clone();

								tauri::async_runtime::spawn(async move {
									let _ = child.wait();
									s.ignore_events.store(false, Ordering::SeqCst);

									if let Some(app_handle) = APP_HANDLE.get() {
										if let Some(window) = app_handle.get_window("main") {
											window
												.set_fullscreen(is_fullscreen)
												.expect("Could not set fullscreen state");
										}
									}
								});
							}
						}
						InputEvent::Menu => {
							let mut orientation = self.orientation.lock().await;
							orientation.menu_active = !orientation.menu_active;
							if orientation.menu_active {
								orientation.menu_index = Some(0);
							} else {
								orientation.menu_index = None;
								orientation.menu_item_index = None;
							}
						}
						InputEvent::Right => {
							let mut lock = self.orientation.lock().await;
							if !lock.menu_active {
								let len = self.all_systems.lock().await.system.len() - 1;
								if lock.system_index >= len {
									lock.system_index = 0;
								} else {
									lock.system_index += 1;
								}

								lock.gamelist_index = 0;
							}
						}
						InputEvent::Left => {
							let mut lock = self.orientation.lock().await;
							if !lock.menu_active {
								let len = self.all_systems.lock().await.system.len() - 1;
								if lock.system_index == 0 {
									lock.system_index = len;
								} else {
									lock.system_index -= 1;
								}

								lock.gamelist_index = 0;
							}
						}
						InputEvent::Up => {
							let mut lock = self.orientation.lock().await;
							if lock.menu_active {
								let len = self.menu().len() - 1;
								if let Some(index) = lock.menu_index {
									if let Some(inner_idx) = lock.menu_item_index {
										if inner_idx == 0 {
											lock.menu_item_index =
												Some(self.settings_menu().len() - 1);
										} else {
											lock.menu_item_index = Some(inner_idx - 1);
										}
									} else if index == 0 {
										lock.menu_index = Some(len);
									} else {
										lock.menu_index = Some(index - 1);
									}
								} else {
									lock.menu_index = Some(0)
								}
							} else {
								let len = self.all_systems.lock().await.system[lock.system_index]
									.gamelist
									.len() - 1;

								if lock.gamelist_index == 0 {
									lock.gamelist_index = len;
								} else {
									lock.gamelist_index -= 1;
								}
							}
						}
						InputEvent::Down => {
							let mut lock = self.orientation.lock().await;
							if lock.menu_active {
								let len = self.menu().len() - 1;

								if let Some(index) = lock.menu_index {
									if let Some(inner_idx) = lock.menu_item_index {
										if inner_idx == self.settings_menu().len() - 1 {
											lock.menu_item_index = Some(0);
										} else {
											lock.menu_item_index = Some(inner_idx + 1);
										}
									} else if index == len {
										lock.menu_index = Some(0);
									} else {
										lock.menu_index = Some(index + 1);
									}
								} else {
									lock.menu_index = Some(0)
								}
							} else {
								let len = self.all_systems.lock().await.system[lock.system_index]
									.gamelist
									.len() - 1;

								if lock.gamelist_index == len {
									lock.gamelist_index = 0;
								} else {
									lock.gamelist_index += 1;
								}
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
