#![allow(deprecated)]

use std::collections::HashMap;
use config::{Config};
use std::sync::mpsc::channel;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;
use std::fs::File;
use std::io::{Write};
use std::process::Command;
use ansi_term::{Color, Style};
use yaml_rust::YamlLoader;

mod cfg;
mod logger;
mod updater;
mod installer;
mod windows;
mod utils;
extern crate dirs;
extern crate winconsole;
extern crate yaml_rust;

static mut SETTINGS: Option<Config> = None;

fn main() {

    set_console_title();

    let mut just_updated = false;

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "--help" || args[1] == "-h" {
            logger::banner();
            println!("Puedes encontrar la documentación para este programa en https://github.com/holasoyender/Projectile\n");
            println!(r#"
    --help, -h: Muestra este mensaje.
    --version, -v: Muestra la versión del programa.
    --config, -c: Muestra el archivo de configuración actual.
            "#);
            std::process::exit(0);
        } else if args[1] == "--version" || args[1] == "-v" {
            logger::banner();
            std::process::exit(0);
        } else if args[1] == "--config" || args[1] == "-c" {
            logger::banner();
            println!("La configuración actual se encuentra en {}", cfg::get_config_filename());
            std::process::exit(0);
        } else if args[1] == "--update" {
            updater::handle_update();
            just_updated = true;
        } else {
            logger::banner();
            println!("No se reconoce el argumento {}", args[1]);
            std::process::exit(1);
        }
    }

    logger::banner();

    let mut just_started = false;
    if !cfg::config_file_exists() {
        just_started = true;
        println!("Parece que es la primera vez que inicias este programa!");
        println!("Puedes obtener toda la información necesaria para mi uso en https://github.com/holasoyender/Projectile");
        println!("A continuación se creará un archivo de configuración por defecto, el cual puedes encontrar en {}\\settings.yml", cfg::get_root_path());
        println!("Tus templates de proyectos se guardarán en la carpeta {}\\projects, pero puedes cambiarla en cualquier momento desde el archivo de configuración", cfg::get_root_path());

        if !installer::install() {
            println!();
            println!("Por favor, agrega la carpeta {} a tu PATH para que Projectile funcione correctamente", cfg::get_root_path());
        }
    }

    if !just_updated && !just_started {
        if updater::check_for_updates() {
            println!("Se ha encontrado una versión más reciente, ¿Quieres actualizar? (Si/No)");

            let mut option = String::new();
            loop {
                print!(">>> ");
                std::io::stdout().lock().flush().unwrap();
                std::io::stdin().read_line(&mut option).unwrap();
                option = option.trim().to_string();
                if option == "Si" || option == "si" || option == "y" || option == "Y" || option == "yes" || option == "YES" {
                    updater::update();
                    break;
                } else if option == "No" || option == "no" || option == "n" || option == "N" {
                    logger::info("Se ha omitido la actualización.");
                    println!();
                    break;
                } else {
                    println!("No se reconoce la opción {}", option);
                    option = String::new();
                }
            }
        } else {
            logger::api("No se han encontrado versiones más recientes.");
            println!();
        }
    }

    if windows::is_app_elevated() {
        logger::warn("El programa se ejecuta como administrador, esto no es recomendado.");
    }

    if !cfg::config_exists() {

        let config_dir = dirs::config_dir();
        let config_path = config_dir.unwrap().join("Projectile");
        if !config_path.exists() {
            std::fs::create_dir_all(&config_path).unwrap();
        }

        let file = File::create(cfg::get_config_filename());
        match file {
            Ok(mut f) => {
                f.write_all(cfg::get_default_config().as_bytes()).unwrap();
            }
            Err(e) => {
                logger::error(format!("No se ha podido crear el archivo de configuración: {:?}", e).as_str());
                std::process::exit(1);
            }
        }

        if just_started {
            std::thread::sleep(Duration::from_secs(60));
            std::process::exit(0);
        }

        let settings = Config::builder()
            .add_source(config::File::with_name(cfg::get_config_filename().as_str()))
            .build();
        match settings {
            Ok(settings) => unsafe {
                SETTINGS = Some(settings);
            }
            Err(_) => {
                logger::error("El archivo de configuración no existe o está corrupto, creando uno nuevo...");
                let file = File::create(cfg::get_config_filename());
                match file {
                    Ok(mut f) => {
                        f.write_all(cfg::get_default_config().as_bytes()).unwrap();
                    }
                    Err(e) => {
                        logger::error(format!("No se ha podido crear el archivo de configuración: {:?}", e).as_str());
                        std::process::exit(1);
                    }
                }
                return;
            }
        }

    } else {

        let settings = Config::builder()
            .add_source(config::File::with_name(cfg::get_config_filename().as_str()))
            .build();
        match settings {
            Ok(settings) => unsafe {
                SETTINGS = Some(settings);
            }
            Err(_) => {
                if !just_started {
                    logger::error("El archivo de configuración no existe o está corrupto, creando uno nuevo...");
                }
                let mut file = File::create(cfg::get_config_filename()).unwrap();

                file.write_all(cfg::get_default_config().as_bytes()).unwrap();
                let settings = Config::builder()
                    .add_source(config::File::with_name(cfg::get_config_filename().as_str()))
                    .build();
                unsafe {
                    SETTINGS = Some(settings.unwrap());
                }
                if just_started {
                    std::thread::sleep(Duration::from_secs(60));
                    std::process::exit(0);
                }
            }
        }
    }

    unsafe {
        match SETTINGS
            .clone()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>() {
            Ok(settings) => {
                if !settings.contains_key("project_path") {
                    logger::error("El archivo de configuración no existe o está corrupto, creando uno nuevo...");
                    let file = File::create(cfg::get_config_filename());
                    match file {
                        Ok(mut f) => {
                            f.write_all(cfg::get_default_config().as_bytes()).unwrap();
                        }
                        Err(e) => {
                            logger::error(format!("No se ha podido crear el archivo de configuración: {:?}", e).as_str());
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(e) => {
                logger::error(format!("Error deserializing settings: {:?}", e).as_str());
            }
        }
    }
    std::thread::spawn(|| {
        let project_path = unsafe {
            SETTINGS
                .clone()
                .unwrap()
                .try_deserialize::<HashMap<String, String>>().unwrap()["project_path"].clone()
        };
        logger::ok(format!("Se ha cargado la configuración desde {}", project_path).as_str());
        let folders = std::fs::read_dir(project_path.as_str());
        let mut projects: Vec<String> = Vec::new();
        let mut projects_names: Vec<String> = Vec::new();
        match folders {
            Err(_) => {
                std::fs::create_dir_all(project_path.as_str()).unwrap();
                let folders_new = std::fs::read_dir(project_path.as_str()).unwrap();
                for folder in folders_new {
                    let folder = folder.unwrap();
                    if folder.path().is_dir() {
                        projects.push(folder.path().to_str().unwrap().to_string());
                        projects_names.push(folder.path().file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
            Ok(folders) => {
                for folder in folders {
                    let folder = folder.unwrap();
                    if folder.path().is_dir() {
                        projects.push(folder.path().to_str().unwrap().to_string());
                        projects_names.push(folder.path().file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
        }
        if projects.len() == 0 {
            println!("Parece que no hay proyectos en el directorio de proyectos.\nLee la documentación para saber cómo crear un proyecto.");
            std::thread::sleep(Duration::from_secs(5));
            std::process::exit(1);
        }

        println!("Se han encontrado {} proyectos en el directorio de proyectos.", projects.len());
        println!("Selecciona un proyecto para empezar (1 - {}):\n", projects.len());
        for i in 0..projects.len() {
            let project_file = std::fs::read_dir(projects[i].as_str()).unwrap();
            let mut found = false;
            for file in project_file {
                let file = file.unwrap();
                if file.path().file_name().unwrap().to_str().unwrap() == "project.yml" {
                    found = true;
                    break;
                }
            }
            if found {
                let project_path = projects[i].clone();
                let project_file = std::fs::read_to_string(project_path + "\\project.yml").unwrap();
                let project_file = YamlLoader::load_from_str(project_file.as_str());
                match project_file {
                    Ok(project_file) => {
                        let file = project_file[0].clone();
                        let project_desc = file["description"].as_str();
                        match project_desc {
                            Some(desc) => {
                                if config_color() {
                                    println!("{}", Style::new().bold().paint(format!(" > {}. {} - {}", i + 1, Color::Green.paint(projects_names[i].as_str()), Color::Blue.paint(desc))));
                                } else {
                                    println!(" > {}. {} - {}", i + 1, projects_names[i], desc);
                                }
                            }
                            None => {
                                if config_color() {
                                    println!("{}", Style::new().bold().paint(format!(" > {}. {}", i + 1, Color::Green.paint(projects_names[i].as_str()))));
                                } else {
                                    println!(" > {}. {}", i + 1, projects_names[i]);
                                }
                            }
                        }
                    }
                    Err(_) => {
                        if config_color() {
                            println!("{}", Style::new().bold().paint(format!(" > {}. {} {}", i + 1, Color::Green.paint(projects_names[i].as_str()), Color::Red.paint("(Error al leer al archivo project.yml)"))));
                        } else {
                            println!(" > {}. {} (Error al leer al archivo project.yml)", i + 1, projects_names[i]);
                        }
                    }
                }
            } else {
                if config_color() {
                    println!("{}", Style::new().bold().paint(format!(" > {}. {} {}", i + 1, Color::Green.paint(projects_names[i].as_str()), Color::Red.paint("(No se ha encontrado el archivo project.yml)"))));
                } else {
                    println!(" > {}. {} (No se ha encontrado el archivo project.yml)", i + 1, projects_names[i]);
                }
            }
        }
        let mut selected_project_raw = String::new();
        let selected_project_name;
        let selected_project_path;
        loop {
            print!(">>> ");
            std::io::stdout().lock().flush().unwrap();
            std::io::stdin().read_line(&mut selected_project_raw).unwrap();
            let selected_project = selected_project_raw.trim().parse::<usize>();
            match selected_project {
                Ok(project) => {
                    if project > 0 && project <= projects.len() {
                        selected_project_name = projects_names[project - 1].clone();
                        selected_project_path = projects[project - 1].clone();
                        break;
                    } else {
                        println!("Selección inválida. Intenta de nuevo.");
                        selected_project_raw = String::new();
                    }
                }
                Err(_) => {
                    println!("La selección debe de ser un número!. Intentalo de nuevo.");
                    selected_project_raw = String::new();
                }
            }
        }

        logger::ok(format!("Cargando el proyecto {}...", selected_project_name).as_str());

        let project_files = std::fs::read_dir(selected_project_path.clone()).unwrap();
        let mut files = Vec::new();
        let mut found = false;
        for file in project_files {
            let file = file.unwrap();
            if file.path().file_name().unwrap().to_str().unwrap() != "project.yml" {
                files.push(file.path().to_str().unwrap().to_string());
            }
            if file.path().file_name().unwrap().to_str().unwrap() == "project.yml" {
                found = true;
            }
        }
        if !found {
            let current_folder = std::fs::read_dir("./").unwrap();
            let mut empty = true;

            if current_folder.count() > 1 {
                empty = false;
            }
            if !empty {
                println!();
                logger::error("El directorio actual no está vacío. Por seguridad este programa solo se puede usar en directorios vacíos.");
                std::thread::sleep(Duration::from_secs(5));
                std::process::exit(1);
            }
            println!("No se ha encontrado el archivo project.yml en el directorio del proyecto. Copiando todos los archivos...");

            for file in files {
                let file_name = file.split("\\").last().unwrap();
                let copy_action = std::fs::copy(file.as_str(), format!("./{}", file_name));
                match copy_action {
                    Ok(_) => {
                        logger::info(format!("Copiado {}", file_name).as_str());
                    }
                    Err(_) => {
                        if utils::copy_folder_content(file.as_str().clone()) {
                            logger::info(format!("Copiado {}", file_name).as_str());
                        } else {
                            logger::error(format!("Copiado {}", file_name).as_str());
                        }
                    }
                }
            }
            logger::ok(format!("Todos los archivos del proyecto {} fueron copiados correctamente!.", selected_project_name).as_str());
            std::thread::sleep(Duration::from_secs(5));
            std::process::exit(0);
        } else {

            let current_folder = std::fs::read_dir("./").unwrap();
            let mut empty = true;

            if current_folder.count() > 1 {
                empty = false;
            }
            if !empty {
                println!();
                logger::error("El directorio actual no está vacío. Por seguridad este programa solo se puede usar en directorios vacíos.");
                std::thread::sleep(Duration::from_secs(5));
                std::process::exit(1);
            }

            let project_path = selected_project_path.clone();
            let project_file = std::fs::read_to_string(project_path + "\\project.yml").unwrap();
            let project_file = YamlLoader::load_from_str(project_file.as_str());
            match project_file {
                Ok(project_file) => {
                    let file = project_file[0].clone();
                    let project_vars = file["vars"].as_vec();
                    match project_vars {
                        Some(vars) => {
                            println!("El proyecto cuenta con {} variable(s), escribe el valor para cada una a continuación.\n", vars.len());
                            let mut vars_setters = Vec::new();
                            let mut vars_names = Vec::new();
                            let mut vars_values = Vec::new();

                            for i in 0..vars.len() {
                                let var_content = vars[i].as_hash().unwrap();
                                let var_setter = var_content.keys().nth(0).unwrap().as_str().unwrap();
                                let var_name = var_content.values().nth(0).unwrap().as_str().unwrap();

                                vars_names.push(var_name.clone());
                                vars_setters.push(var_setter.clone());
                            }

                            for i in 0..vars_names.len() {
                                let mut value = String::new();
                                let var_name = vars_names[i].clone();
                                let var_setter = vars_setters[i].clone();
                                loop {
                                    print!("{} > ", var_name);
                                    std::io::stdout().lock().flush().unwrap();
                                    std::io::stdin().read_line(&mut value).unwrap();
                                    if value.trim().len() > 0 {
                                        vars_values.push(value.trim().to_string());
                                        println!("Se ha guardado el valor '{}' para la variable {}.", value.trim(), var_setter);
                                        break;
                                    } else {
                                        vars_values.push("None".to_string());
                                        println!("El valor de la variable {} se ha establecido como None.", var_setter);
                                        break;
                                    }
                                }
                            }
                            logger::ok("Todas las variables han sido guardadas correctamente! Copiando archivos...");
                            for i in 0..files.len() {
                                let mut file_name = files[i].split("\\").last().unwrap().to_string();
                                let file_content = std::fs::read_to_string(files[i].as_str());
                                match file_content {
                                    Ok(mut content) => {
                                        for j in 0..vars_setters.len() {
                                            let setter = vars_setters[j].clone();
                                            let value = vars_values[j].clone();

                                            content = content.replace(format!(r#"{{{{{}}}}}"#, setter).as_str(), value.as_str());
                                            file_name = file_name.replace(format!(r#"{{{{{}}}}}"#, setter).as_str(), value.as_str());
                                        }
                                        std::fs::write(format!("./{}", file_name).as_str(), content).unwrap();
                                        let percent = (i as f32 / files.len() as f32) * 100.0;
                                        logger::info(format!("Copiado {} ({}%)", file_name, percent).as_str());
                                    }
                                    Err(_) => {
                                        if utils::copy_folder_content_with_vars( files[i].as_str().clone(), &vars_setters, &vars_values) {
                                            logger::info(format!("Copiado {}", file_name).as_str());
                                        } else {
                                            logger::error(format!("No se ha podido copiar el archivo {}", file_name).as_str());
                                        }
                                    }
                                }
                            }
                            logger::ok("Todos los archivos fueron copiados correctamente!.");

                            let project_scripts = file["scripts"].as_vec();
                            match project_scripts {
                                Some(scripts) => {
                                    println!("Ejecutando {} script(s)...", scripts.len());
                                    for i in 0..scripts.len() {
                                        let script = scripts[i].as_str().unwrap();
                                        let output = Command::new("cmd")
                                            .arg("/c")
                                            .arg(script)
                                            .output()
                                            .expect("Error al ejecutar el script");
                                        println!("{}", String::from_utf8_lossy(&output.stdout));
                                        println!("{}", String::from_utf8_lossy(&output.stderr));

                                        if i == scripts.len() - 1 {
                                            logger::ok("Todos los scripts han sido ejecutados correctamente! Cerrando consola...");
                                            std::thread::sleep(Duration::from_secs(5));
                                            std::process::exit(0);
                                        }
                                    }
                                },
                                None => {
                                    logger::info("No se han encontrado scripts en el archivo project.yml. Cerrando consola...");
                                    std::thread::sleep(Duration::from_secs(5));
                                    std::process::exit(0);
                                }
                            }
                            println!("{:?}", vars_values);

                        }
                        None => {
                            let current_folder = std::fs::read_dir("./").unwrap();
                            let mut empty = true;

                            if current_folder.count() > 1 {
                                empty = false;
                            }
                            if !empty {
                                println!();
                                logger::error("El directorio actual no está vacío. Por seguridad este programa solo se puede usar en directorios vacíos.");
                                std::thread::sleep(Duration::from_secs(5));
                                std::process::exit(1);
                            }

                            logger::error("No se han encontrado variables en el archivo project.yml. Copiando todos los archivos...");

                            for file in files {
                                let file_name = file.split("\\").last().unwrap();
                                let copy_action = std::fs::copy(file.as_str(), format!("./{}", file_name));
                                match copy_action {
                                    Ok(_) => {
                                        logger::info(format!("Copiado {}", file_name).as_str());
                                    }
                                    Err(_) => {
                                        if utils::copy_folder_content(file.as_str().clone()) {
                                            logger::info(format!("Copiado {}", file_name).as_str());
                                        } else {
                                            logger::error(format!("Copiado {}", file_name).as_str());
                                        }
                                    }
                                }
                            }
                            logger::ok(format!("Todos los archivos del proyecto {} fueron copiados correctamente!.", selected_project_name).as_str());

                            let project_scripts = file["scripts"].as_vec();
                            match project_scripts {
                                Some(scripts) => {
                                    println!("Ejecutando {} script(s)...", scripts.len());
                                    for i in 0..scripts.len() {
                                        let script = scripts[i].as_str().unwrap();
                                        let output = Command::new("cmd")
                                            .arg("/c")
                                            .arg(script)
                                            .output()
                                            .expect("Error al ejecutar el script");
                                        println!("{}", String::from_utf8_lossy(&output.stdout));
                                        println!("{}", String::from_utf8_lossy(&output.stderr));

                                        if i == scripts.len() -1 {
                                            logger::ok("Todos los scripts han sido ejecutados correctamente! Cerrando consola...");
                                            std::thread::sleep(Duration::from_secs(5));
                                            std::process::exit(0);
                                        }
                                    }
                                },
                                None => {
                                    logger::info("No se han encontrado scripts en el archivo project.yml. Cerrando consola...");
                                    std::thread::sleep(Duration::from_secs(5));
                                    std::process::exit(0);
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    let current_folder = std::fs::read_dir("./").unwrap();
                    let mut empty = true;

                    if current_folder.count() > 1 {
                        empty = false;
                    }
                    if !empty {
                        println!();
                        logger::error("El directorio actual no está vacío. Por seguridad este programa solo se puede usar en directorios vacíos.");
                        std::thread::sleep(Duration::from_secs(5));
                        std::process::exit(1);
                    }

                    logger::error("No se ha podido leer el archivo project.yml. Copiando todos los archivos...");

                    for file in files {
                        let file_name = file.split("\\").last().unwrap();
                        let copy_action = std::fs::copy(file.as_str(), format!("./{}", file_name));
                        match copy_action {
                            Ok(_) => {
                                logger::info(format!("Copiado {}", file_name).as_str());
                            }
                            Err(_) => {
                                if utils::copy_folder_content(file.as_str().clone()) {
                                    logger::info(format!("Copiado {}", file_name).as_str());
                                } else {
                                    logger::error(format!("Copiado {}", file_name).as_str());
                                }
                            }
                        }
                    }
                    logger::ok(format!("Todos los archivos del proyecto {} fueron copiados correctamente!.", selected_project_name).as_str());
                    std::thread::sleep(Duration::from_secs(5));
                    std::process::exit(0);

                }
            }

        }

    });
    watch()
}

fn watch() {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher
        .watch(cfg::get_config_filename(), RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                logger::info("La configuración se ha modificado, recargando...");
                let mut settings = Config::default();
                match settings.merge(config::File::with_name(cfg::get_config_filename().as_str())) {
                    Ok(_) => {
                        logger::ok("Configuración recargada!");
                    }
                    Err(e) => {
                        logger::error(format!("Ha ocurrido un error al recargar la configuración: \x1b[31m{:?}\x1b[0m", e).as_str());
                    }
                }
            }

            Err(e) => logger::error(format!("watch error: {:?}", e).as_str()),

            _ => {
                // Ignore event
            }
        }
    }
}

fn set_console_title() {
    winconsole::console::set_title(format!("Projectile v{}", env!("CARGO_PKG_VERSION")).as_str()).unwrap();
}

pub fn config_color() -> bool {
    unsafe {
        let settings = SETTINGS.clone();
        return match settings {
            Some(_settings) => {
                let settings = _settings.try_deserialize::<HashMap<String, String>>().unwrap();
                if settings.contains_key("color") {
                    if settings.get("color").unwrap() == "true" {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            None => {
                true
            }
        }
    }
}