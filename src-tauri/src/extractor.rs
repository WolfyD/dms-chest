use std::fs;
use sevenz_rust;
use tauri::Manager;

pub fn extract_7z(app: &tauri::AppHandle) -> Result<(), String> {
    println!("Starting 7z extraction check...");
    
    // Get the app's data directory
    let app_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    println!("App data directory: {:?}", app_dir);
    
    // Check if the JSON file already exists
    let json_path = app_dir.join("wn.json");
    if json_path.exists() {
        println!("JSON file already exists at: {:?}, skipping extraction", json_path);
        return Ok(());
    }

    println!("JSON file not found, proceeding with extraction...");
    
    // Create executables directory if it doesn't exist
    let executables_dir = app_dir;
    println!("Creating executables directory at: {:?}", executables_dir);
    fs::create_dir_all(&executables_dir)
        .map_err(|e| format!("Failed to create executables directory: {}", e))?;

    // Get the path to the 7z file in the static folder
    let resource_dir = app.path().resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;
    println!("Resource directory: {:?}", resource_dir);
    
    let archive_path = resource_dir.join("static").join("wn.7z");
    println!("Looking for archive at: {:?}", archive_path);
    
    // Check if the archive exists
    if !archive_path.exists() {
        return Err(format!("Archive not found at: {:?}", archive_path));
    }

    println!("Found archive, starting extraction...");
    // Extract the 7z file
    sevenz_rust::decompress_file(
        archive_path,
        executables_dir,
    ).map_err(|e| format!("Failed to extract 7z file: {}", e))?;

    println!("Extraction completed successfully!");
    Ok(())
} 