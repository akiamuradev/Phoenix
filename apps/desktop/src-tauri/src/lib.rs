use phoenix_core::commands;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceSummary {
    path: String,
    format: String,
    schema_version: u32,
}

#[tauri::command]
fn create_workspace(path: String) -> Result<WorkspaceSummary, String> {
    let metadata = commands::create_workspace(&path).map_err(|error| error.to_string())?;
    Ok(WorkspaceSummary {
        path,
        format: metadata.format,
        schema_version: metadata.schema_version,
    })
}

#[tauri::command]
fn open_workspace(path: String) -> Result<WorkspaceSummary, String> {
    let metadata = commands::open_workspace(&path).map_err(|error| error.to_string())?;
    Ok(WorkspaceSummary {
        path,
        format: metadata.format,
        schema_version: metadata.schema_version,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Starts the Phoenix desktop runtime.
///
/// # Panics
///
/// Panics when Tauri cannot initialize or run the application event loop.
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_workspace, open_workspace])
        .run(tauri::generate_context!())
        .expect("error while running Phoenix");
}
