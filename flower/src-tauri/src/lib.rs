use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use tauri_plugin_dialog::DialogExt;

#[derive(serde::Serialize)]
struct FileData {
    path: String,
    content: String,
}

#[derive(serde::Serialize, Clone)]
struct NoteFile {
    path: String,
    name: String,
    updated_at: u64,
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

#[tauri::command]
fn list_notes(dir: String) -> Result<Vec<NoteFile>, String> {
    let dir_path = PathBuf::from(&dir);
    if !dir_path.is_dir() {
        return Err(format!("不是有效的目录: {}", dir));
    }
    let mut notes: Vec<NoteFile> = fs::read_dir(&dir_path)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let ext = path.extension()?.to_str()?;
            if ext != "md" {
                return None;
            }
            let name = path.file_stem()?.to_str()?.to_string();
            let updated_at = entry
                .metadata()
                .ok()?
                .modified()
                .ok()?
                .duration_since(UNIX_EPOCH)
                .ok()?
                .as_secs();
            Some(NoteFile {
                path: path.to_string_lossy().into(),
                name,
                updated_at,
            })
        })
        .collect();
    notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(notes)
}

#[tauri::command]
fn create_note(dir: String) -> Result<NoteFile, String> {
    let ts = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let name = format!("Untitled-{}", ts);
    let path = PathBuf::from(&dir).join(format!("{}.md", &name));
    fs::write(&path, "").map_err(|e| e.to_string())?;
    Ok(NoteFile {
        path: path.to_string_lossy().into(),
        name,
        updated_at: ts,
    })
}

#[tauri::command]
fn read_note(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_note(path: String) -> Result<(), String> {
    fs::remove_file(&path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            open_file,
            save_file,
            save_file_as,
            list_notes,
            create_note,
            read_note,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
