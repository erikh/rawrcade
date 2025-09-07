use super::{App, Orientation, System};
use tauri::State;

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
