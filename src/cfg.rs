extern crate dirs;

pub fn config_exists()-> bool {
    let config_dir = dirs::config_dir();
    let config_path = config_dir.unwrap().join("Projectile");
    return config_path.exists();
}
pub fn config_file_exists() -> bool {
    let config_dir = dirs::config_dir();
    let file = config_dir.unwrap().join("Projectile").join("settings.yaml");
    return file.exists();
}
pub fn get_config_filename() -> String {
    let config_dir = dirs::config_dir();
    let config_path = config_dir.unwrap().join("Projectile");
    return config_path.to_str().unwrap().to_string()+"\\settings.yaml";
}
pub fn get_update_path() -> String {
    let config_dir = dirs::config_dir();
    let config_path = config_dir.unwrap().join("Projectile").join("Updater");
    if !config_path.exists() {
        std::fs::create_dir_all(config_path.clone()).unwrap();
    }
    return config_path.to_str().unwrap().to_string();
}
pub fn get_root_path() -> String {
    let config_dir = dirs::config_dir();
    let config_path = config_dir.unwrap().join("Projectile");
    return config_path.to_str().unwrap().to_string();
}
pub fn get_default_config() -> String {
    let config_dir = dirs::config_dir();
    let config_path = config_dir.unwrap().join("Projectile");
    return format!(r#"
project_path: "{}"
color: true "#, config_path.to_str().unwrap().to_string().replace("\\", "\\\\")+"\\\\projects");
}