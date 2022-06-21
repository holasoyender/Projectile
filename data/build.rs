extern crate static_vcruntime;
extern crate winres;
//extern crate chrono;

fn main() {
    if cfg!(target_os = "windows") {

        // let timestamp = chrono::Local::now().format("%d/%m/%Y %H:%M:%S");
        // std::env::set_var("COMPILE_TIME", timestamp.to_string());

        let mut res = winres::WindowsResource::new();
        res.set_icon("data/icon.ico")
            .set("ProductName", "Projectile")
            .set("FileDescription", "Gestiona tus templates de proyectos!")
            .set("LegalCopyright", "Copyright (c) 2022 holasoyender")
            .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        res.compile().unwrap();
    }
}