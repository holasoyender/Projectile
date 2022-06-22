use crate::cfg;

pub fn install() -> bool {

    if !cfg::config_exists() {
        //create all folders of get_root_path()
        let root_path = cfg::get_root_path();
        if !std::path::Path::new(&root_path).exists() {
            std::fs::create_dir_all(&root_path).unwrap();
        }
    }
    //Copy itself to the folder
    let config_path = cfg::get_root_path();
    //Get the exe path
    let exe_path = std::env::current_exe().unwrap();
    std::fs::copy(exe_path, config_path.clone()+"\\projectile.exe").unwrap();

    let system_path = std::env::var("PATH").unwrap();
    return system_path.contains(&config_path);
}