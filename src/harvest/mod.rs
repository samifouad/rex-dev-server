use std::fs;
use std::path::Path;

pub struct AppData {
    pub app_root_files: Vec<String>,
    pub pages_files: Vec<String>,
    pub api_files: Vec<String>,
}

pub fn app_folder(app_folder_path: &str) -> std::io::Result<AppData> {
    let mut app_data = AppData {
        app_root_files: Vec::new(),
        pages_files: Vec::new(),
        api_files: Vec::new(),
    };

    let entries = fs::read_dir(app_folder_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();

            if path.starts_with(Path::new(app_folder_path).join("pages")) {
                if let Ok(relative_path) = path.strip_prefix(app_folder_path) {
                    app_data.pages_files.push(relative_path.to_string_lossy().to_string());
                }
            } else {
                app_data.app_root_files.push(file_name);
            }

        } else if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();

            match dir_name.as_ref() {
                "static" => {
                    collect_files(&path, &mut app_data, dir_name.as_ref())?;
                }
                "pages" => {
                    traverse_recursive(&path, &mut app_data.pages_files, &path)?;
                }
                "api" => {
                    traverse_recursive(&path, &mut app_data.api_files, &path)?;
                }
                _ => (),
            }
        }
    }

    Ok(app_data)
}

fn collect_files(folder_path: &Path, app_data: &mut AppData, category: &str) -> std::io::Result<()> {
    if let Ok(entries) = fs::read_dir(folder_path) {
        let files = match category {
            "api" => &mut app_data.api_files,
            _ => return Ok(()),
        };

        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    if let Some(file_name) = entry.path().file_name() {
                        files.push(file_name.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    Ok(())
}

fn traverse_recursive(path: &Path, files: &mut Vec<String>, root_path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Ok(relative_path) = entry_path.strip_prefix(root_path) {
                    files.push(relative_path.to_string_lossy().to_string());
                }
            } else if entry_path.is_dir() {
                traverse_recursive(&entry_path, files, root_path)?;
            }
        }
    }
    Ok(())
}