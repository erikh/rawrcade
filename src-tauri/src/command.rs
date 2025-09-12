use super::{App, Orientation, System};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;
use thiserror::Error;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
	#[default]
	#[serde(rename = "image")]
	Image,
	#[serde(rename = "thumbnail")]
	Thumbnail,
	#[serde(rename = "video")]
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
) -> std::result::Result<Option<PathBuf>, ()> {
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
	Ok(filename)
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
