use std::fs;
use std::path::PathBuf;
use tauri_plugin_dialog::DialogExt;

#[derive(serde::Serialize)]
struct FileData {
    path: String,
    content: String,
}

#[tauri::command]
fn open_file(app: tauri::AppHandle) -> Result<FileData, String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("Markdown / Text", &["md", "txt", "markdown"])
        .blocking_pick_file();

    match file_path {
        None => Err("cancelled".into()),
        Some(p) => {
            let path = PathBuf::from(p.to_string());
            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            Ok(FileData {
                path: path.to_string_lossy().into(),
                content,
            })
        }
    }
}

#[tauri::command]
fn save_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_file_as(app: tauri::AppHandle, content: String) -> Result<String, String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md"])
        .blocking_save_file();

    match file_path {
        None => Err("cancelled".into()),
        Some(p) => {
            let path = PathBuf::from(p.to_string());
            fs::write(&path, &content).map_err(|e| e.to_string())?;
            Ok(path.to_string_lossy().into())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![open_file, save_file, save_file_as])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
