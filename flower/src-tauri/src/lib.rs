use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use base64::Engine;
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

#[derive(serde::Serialize, Clone)]
struct VaultEntry {
    name: String,
    path: String,
    entry_type: String,        // "dir" | "file"
    children: Vec<VaultEntry>,
    updated_at: u64,
}

fn scan_dir(dir: &std::path::Path) -> Result<Vec<VaultEntry>, String> {
    let mut dirs: Vec<VaultEntry> = Vec::new();
    let mut files: Vec<VaultEntry> = Vec::new();

    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();

        // Skip hidden files/dirs
        if name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            let children = scan_dir(&path)?;
            dirs.push(VaultEntry {
                name,
                path: path.to_string_lossy().into(),
                entry_type: "dir".into(),
                children,
                updated_at: 0,
            });
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let updated_at = entry
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let stem = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
            files.push(VaultEntry {
                name: stem.to_string(),
                path: path.to_string_lossy().into(),
                entry_type: "file".into(),
                children: vec![],
                updated_at,
            });
        }
    }

    dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    files.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    dirs.extend(files);
    Ok(dirs)
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
fn list_vault_tree(dir: String) -> Result<Vec<VaultEntry>, String> {
    let dir_path = std::path::Path::new(&dir);
    if !dir_path.is_dir() {
        return Err(format!("不是有效的目录: {}", dir));
    }
    scan_dir(dir_path)
}

#[tauri::command]
fn create_note(dir: String, subdir: Option<String>) -> Result<NoteFile, String> {
    let ts = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let name = format!("Untitled-{}", ts);
    let mut target_dir = PathBuf::from(&dir);
    if let Some(ref sd) = subdir {
        target_dir.push(sd);
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }
    let path = target_dir.join(format!("{}.md", &name));
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

#[tauri::command]
fn import_note(vault_dir: String, source_path: String, subdir: Option<String>) -> Result<NoteFile, String> {
    let src = PathBuf::from(&source_path);
    let stem = src.file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("imported");
    let mut target_dir = PathBuf::from(&vault_dir);
    if let Some(ref sd) = subdir {
        target_dir.push(sd);
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }
    let mut dest = target_dir.join(format!("{}.md", stem));

    if dest.exists() {
        let mut counter = 1;
        loop {
            dest = target_dir.join(format!("{} ({}).md", stem, counter));
            if !dest.exists() {
                break;
            }
            counter += 1;
        }
    }

    fs::copy(&src, &dest).map_err(|e| e.to_string())?;
    let updated_at = dest.metadata()
        .map_err(|e| e.to_string())?
        .modified()
        .map_err(|e| e.to_string())?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(NoteFile {
        path: dest.to_string_lossy().into(),
        name: dest.file_stem().and_then(|n| n.to_str()).unwrap_or("imported").to_string(),
        updated_at,
    })
}

#[tauri::command]
fn rename_note(old_path: String, new_name: String) -> Result<NoteFile, String> {
    let old = std::path::Path::new(&old_path);
    let parent = old.parent().ok_or("无效的文件路径")?;
    let new_path = parent.join(format!("{}.md", &new_name));

    if new_path.exists() {
        return Err(format!("「{}」已存在", new_name));
    }

    std::fs::rename(&old_path, &new_path).map_err(|e| e.to_string())?;

    let content = std::fs::read_to_string(&new_path).map_err(|e| e.to_string())?;
    let new_heading = format!("# {}", &new_name);
    let first_line_end = content.find('\n').unwrap_or(content.len());
    let first_line = &content[..first_line_end];

    let updated_content = if first_line == new_heading {
        content
    } else if first_line.starts_with("# ") {
        new_heading + &content[first_line_end..]
    } else {
        format!("{}\n\n{}", new_heading, content)
    };

    std::fs::write(&new_path, &updated_content).map_err(|e| e.to_string())?;

    let updated_at = new_path.metadata()
        .map_err(|e| e.to_string())?
        .modified()
        .map_err(|e| e.to_string())?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(NoteFile {
        path: new_path.to_string_lossy().into(),
        name: new_name,
        updated_at,
    })
}

#[tauri::command]
fn create_folder(dir: String, name: String) -> Result<(), String> {
    let path = PathBuf::from(&dir).join(&name);
    if path.exists() {
        return Err(format!("「{}」已存在", name));
    }
    fs::create_dir(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_folder(path: String) -> Result<(), String> {
    fs::remove_dir_all(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn rename_folder(old_path: String, new_name: String) -> Result<(), String> {
    let old = std::path::Path::new(&old_path);
    let parent = old.parent().ok_or("无效的文件路径")?;
    let new_path = parent.join(&new_name);

    if new_path.exists() {
        return Err(format!("「{}」已存在", new_name));
    }

    std::fs::rename(&old_path, &new_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_image_base64(path: String) -> Result<String, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let ext = std::path::Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let mime = match ext {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        _ => "image/png",
    };
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, b64))
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
            list_vault_tree,
            create_note,
            read_note,
            delete_note,
            read_image_base64,
            import_note,
            rename_note,
            create_folder,
            delete_folder,
            rename_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
