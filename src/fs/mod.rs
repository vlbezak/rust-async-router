use crate::Result;

// Works with std Errors
pub fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs
        ::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e|
            e
                .file_type()
                .map(|ft| ft.is_file())
                .unwrap_or(false)
        )
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}


// Works with String error (Into)
pub fn list_files_with_str_error(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs
        ::read_dir(path)
        .map_err(|ex| "error while reading dir")?
        .filter_map(|re| re.ok())
        .filter(|e|
            e
                .file_type()
                .map(|ft| ft.is_file())
                .unwrap_or(false)
        )
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}

// Works with Custom Error (Into)
pub fn list_files_with_custom_error(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs
        ::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e|
            e
                .file_type()
                .map(|ft| ft.is_file())
                .unwrap_or(false)
        )
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

        if files.is_empty() {
            return Err("Cannot list empty folder".into());
        }

    Ok(files)
}