use std::path::PathBuf;

use thiserror::Error;
use tokio::io::AsyncReadExt;

use super::{App, Orientation, System};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
	#[default]
	Image,
	Thumbnail,
	Video,
}

#[derive(Debug, Error)]
pub enum AssetError {
	#[error("this asset does not have an associated path")]
	NotExist,
	#[error("i/o error")]
	IO(#[from] std::io::Error),
}

#[tauri::command]
pub async fn current_asset(
	state: State<'_, App>, asset_type: AssetType,
) -> std::result::Result<Option<Vec<u8>>, ()> {
	let systems = state.all_systems.lock().await.clone();
	let orientation = state.orientation.lock().await;
	let current_system = &systems[orientation.system_index];
	let current_game =
		&current_system.gamelist[orientation.gamelist_index];
	let filename: Option<PathBuf> = match asset_type {
		AssetType::Image => current_game.image.clone(),
		AssetType::Thumbnail => current_game.thumbnail.clone(),
		AssetType::Video => current_game.video.clone(),
	};

	if let Some(filename) = filename {
		let res = tokio::fs::OpenOptions::new()
			.read(true)
			.open(filename)
			.await;
		match res {
			Ok(mut f) => {
				let mut v = Vec::new();
				let res = f.read_to_end(&mut v).await;
				match res {
					Ok(_) => Ok(Some(v)),
					_ => Ok(None),
				}
			}
			_ => Ok(None),
		}
	} else {
		Ok(None)
	}
}

#[tauri::command]
pub async fn all_systems(
	state: State<'_, App>,
) -> std::result::Result<Vec<System>, ()> {
	Ok(state.all_systems.clone().lock_owned().await.clone())
}

#[tauri::command]
pub async fn current_orientation(
	state: State<'_, App>,
) -> std::result::Result<Orientation, ()> {
	Ok(state.orientation.clone().lock_owned().await.clone())
}
