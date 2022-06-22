use crate::cfg;

pub fn install() -> bool {
    //Copy itself to the folder
    let config_path = cfg::get_root_path();
    //Get the exe path
    let exe_path = std::env::current_exe().unwrap();
    std::fs::copy(exe_path, config_path.clone()+"\\projectile.exe").unwrap();

    let system_path = std::env::var("PATH").unwrap();
    return system_path.contains(&config_path);
}