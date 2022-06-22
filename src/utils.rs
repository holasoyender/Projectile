use crate::logger;

pub fn copy_folder_content(folder_path: &str) -> bool {
    let folder_path_is_folder = std::fs::metadata(folder_path).unwrap().is_dir();
    return if folder_path_is_folder {
        let folder_content = std::fs::read_dir(folder_path).unwrap();
        let folder_name = folder_path.split("\\").last().unwrap();
        let create_folder_action = std::fs::create_dir(format!("./{}", folder_name));
        match create_folder_action {
            Ok(_) => {
                logger::info(format!("Creado el directorio {}", folder_name).as_str());
                for file in folder_content {
                    let file = file.unwrap();
                    let file_path = file.path();
                    let file_name = file_path.to_str().unwrap().split("\\").last().unwrap();
                    let copy_action = std::fs::copy(file_path.to_str().unwrap(), format!("./{}/{}", folder_name, file_name.clone()));
                    match copy_action {
                        Ok(_) => {
                            logger::info(format!("Copiado {}\\{}", folder_name, file_name).as_str());
                        }
                        Err(_) => {
                            if copy_folder_content(format!("{}/{}", folder_path, file_name).as_str()) {
                                logger::info(format!("Copiado {}", file_name).as_str());
                            } else {
                                logger::error(format!("Error al copiar el archivo {}", file_name).as_str());
                            }
                        }
                    }
                }
            }
            Err(_) => {
                logger::error(format!("No se ha podido crear el directorio {}", folder_name).as_str());
                return false;
            }
        }
        true
    } else {
        false
    }
}

pub fn copy_folder_content_with_vars(folder_path: &str, vars_setters: &Vec<&str>, vars_values: &Vec<String>) -> bool {
    let folder_path_is_folder = std::fs::metadata(folder_path).unwrap().is_dir();
    return if folder_path_is_folder {
        let folder_content = std::fs::read_dir(folder_path).unwrap();
        let folder_name = folder_path.split("\\").last().unwrap();
        let create_folder_action = std::fs::create_dir(format!("./{}", folder_name));
        match create_folder_action {
            Ok(_) => {
                logger::info(format!("Creado el directorio {}", folder_name).as_str());
                 for file in folder_content {
                     let file = file.unwrap();
                     let file_path = file.path();
                     let file_name = file_path.to_str().unwrap().split("\\").last().unwrap();
                     let file_content = std::fs::read_to_string(file_path.to_str().unwrap());
                     match file_content {
                         Ok(mut content) => {
                             let mut name = String::new();
                             for j in 0..vars_setters.len() {
                                 let setter = vars_setters[j].clone();
                                 let value = vars_values[j].clone();

                                 content = content.replace(format!(r#"{{{{{}}}}}"#, setter).as_str(), value.as_str());
                                 name = file_name.replace(format!(r#"{{{{{}}}}}"#, setter).as_str(),value.as_str());
                             }
                             std::fs::write(format!("./{}/{}", folder_name, name), content).unwrap();
                             logger::info(format!("Copiado {}", name).as_str());
                         },
                         Err(_) => {
                             if copy_folder_content_with_vars(format!("{}/{}", folder_path, file_name).as_str(), vars_setters, vars_values) {
                                 logger::info(format!("Copiado {}", file_name).as_str());
                             } else {
                                 logger::error(format!("Error al copiar el archivo {}", file_name).as_str());
                                 return false;
                             }
                         }
                     }
                 }
            },
            Err(_) => {
                logger::error(format!("No se ha podido crear el directorio {}", folder_name).as_str());
                return false;
            }
        }

        true
    } else {
        false
    }
}