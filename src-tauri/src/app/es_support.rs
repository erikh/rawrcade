use crate::{Game, GameList, SystemList};
use anyhow::Result;

pub(crate) fn load_es() -> Result<SystemList> {
	let root = dirs::home_dir().unwrap_or("/".into()).join(".rawrcade");
	let mut all_systems = SystemList::from_file(root.join("es_systems.cfg"))
		.expect("es_systems.cfg missing at ~/.rawrcade");

	let gamelist_dir = root.join("gamelists");

	if let Ok(dat) = std::fs::metadata(&gamelist_dir) {
		if dat.is_dir() {
			let dir = std::fs::read_dir(&gamelist_dir)?;
			for item in dir {
				if let Ok(item) = item {
					if item.metadata()?.is_dir() {
						let list = item.path().join("gamelist.xml");
						if std::fs::exists(&list)? {
							let gamelist = GameList::from_file(list)
								.unwrap()
								.game
								.iter()
								.map(|x| Into::<Game>::into(x.clone()))
								.collect::<Vec<Game>>();

							if !gamelist.is_empty() {
								let name = item.file_name();
								let name = name.to_string_lossy();
								let system = all_systems.system.iter_mut().find_map(|x| {
									if x.name.to_lowercase() == name.to_lowercase() {
										Some(x)
									} else {
										None
									}
								});

								if let Some(system) = system {
									system.gamelist = gamelist;
								}
							}
						}
					}
				}
			}
		} else {
			panic!(
				"~/.rawrcade/gamelists is not a directory. If you symlinked it, please symlink the full ES-DE or .emulationstation directory."
			)
		}
	}

	all_systems.system = all_systems
		.system
		.iter()
		.filter_map(|x| {
			if x.gamelist.is_empty() {
				None
			} else {
				Some(x.clone())
			}
		})
		.collect();

	Ok(all_systems)
}
