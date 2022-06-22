extern crate static_vcruntime;
extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {

        static_vcruntime::metabuild();

        let mut res = winres::WindowsResource::new();
        res.set_icon("data/icon.ico")
            .set("ProductName", "Projectile")
            .set("LegalCopyright", "Copyright (c) 2022 holasoyender")
            .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        res.compile().unwrap();
    }
}