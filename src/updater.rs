use crate::cfg;
use crate::logger;
use std::fs::{remove_file, File};
use std::path::Path;
use std::process::Command;
use std::{fs, io, thread, time::Duration};
use zip::ZipArchive;

pub fn check_for_updates() -> bool {
    logger::api("Buscando actualizaciones...");
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://projectile-api.kirobot.cc/api.json",).trim())
        .send();

    return match response {
        Err(_) => false,
        Ok(response) => {
            let body = response.text().unwrap();
            let parsed = json::parse(&body);
            match parsed {
                Err(_) => false,
                Ok(parsed) => {
                    let version = parsed["version"].as_str().unwrap();
                    let version_number = version.replace(".", "").parse::<u64>().unwrap();
                    let current_version_number = env!("CARGO_PKG_VERSION")
                        .replace(".", "")
                        .parse::<u64>()
                        .unwrap();
                    return version_number > current_version_number;
                }
            }
        }
    };
}

pub fn update() {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://projectile-api.kirobot.cc/api.json",).trim())
        .send();

    match response {
        Ok(response) => {
            let body = response.text().unwrap();
            let parsed = json::parse(&body).unwrap();

            let file_url = parsed["url"].as_str().unwrap();

            if file_url != "" {
                logger::api("Descargando actualización...");
                let mut file = File::create(cfg::get_update_path() + "./update.zip").unwrap();
                let mut response = client.get(file_url).send().unwrap();
                response.copy_to(&mut file).unwrap();
                logger::api("Actualización descargada, instalando nueva versión...");
                thread::sleep(Duration::from_millis(100));
                let mut zip =
                    ZipArchive::new(File::open(cfg::get_update_path() + "./update.zip").unwrap())
                        .unwrap();

                for i in 0..zip.len() {
                    let mut file = zip.by_index(i).unwrap();

                    let out_path = Path::new(cfg::get_update_path().as_str()).join(file.name());
                    if file.name().ends_with('/') {
                        fs::create_dir_all(out_path).unwrap();
                    } else {
                        let mut outfile = File::create(out_path).unwrap();
                        io::copy(&mut file, &mut outfile).unwrap();
                    }
                }
                remove_file(cfg::get_update_path() + "./update.zip").unwrap();
                logger::api("Actualización instalada, reiniciando...");

                thread::sleep(Duration::from_millis(1000));

                let path = cfg::get_update_path() + "./projectile.exe";
                Command::new("cmd")
                    .args([
                        "/C",
                        "start",
                        path.as_str(),
                        "--update"
                    ])
                    .spawn()
                    .unwrap();

                thread::sleep(Duration::from_millis(500));

                std::process::exit(0);
            } else {
                logger::error(
                    "No se ha podido completar la actualización, prueba en otro momento.",
                );
            }
        }
        Err(_e) => {
            logger::error("No se ha podido completar la actualización, prueba en otro momento.");
        }
    }
}

pub fn handle_update() {
    println!("OK");
    fs::copy(
        cfg::get_update_path() + "./projectile.exe",
        cfg::get_root_path() + "./projectile.exe",
    ).unwrap();
}
