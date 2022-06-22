use ansi_term::Colour;
use crate::config_color;
use build_time::build_time_local;

pub fn banner() {
    if config_color() {
            println!(r#"{}
Versión: {}. Compilado en: {}. Creado por: {}.
"#, Colour::Green.paint(r#"
    ____               _           __  _ __
   / __ \_________    (_)__  _____/ /_(_) /__
  / /_/ / ___/ __ \  / / _ \/ ___/ __/ / / _ \
 / ____/ /  / /_/ / / /  __/ /__/ /_/ / /  __/
/_/   /_/   \____/_/ /\___/\___/\__/_/_/\___/
                /___/"#)
                     , Colour::Red.paint(env!("CARGO_PKG_VERSION")), Colour::Red.paint(build_time_local!("%Y/%m/%d %H:%M:%S")), Colour::Red.paint("holasoyender"));
        } else {
            println!(r#"
    ____               _           __  _ __
   / __ \_________    (_)__  _____/ /_(_) /__
  / /_/ / ___/ __ \  / / _ \/ ___/ __/ / / _ \
 / ____/ /  / /_/ / / /  __/ /__/ /_/ / /  __/
/_/   /_/   \____/_/ /\___/\___/\__/_/_/\___/
                /___/
Versión: {}. Compilado en: {}. Creado por: holasoyender.
"#, env!("CARGO_PKG_VERSION"), build_time_local!("%Y/%m/%d %H:%M:%S"));
        }
}

pub fn ok(msg: &str) {
    if config_color() {
        println!("[ {} ] - {}", Colour::Green.paint("OK"), msg);
    } else {
        println!("[ OK ] {}", msg);
    }
}
pub fn error(msg: &str) {
    if config_color() {
        println!("[ {} ] - {}", Colour::Red.paint("ERROR"), msg);
    } else {
        println!("[ ERROR ] {}", msg);
    }
}
pub fn info(msg: &str) {
    if config_color() {
        println!("[ {} ] - {}", Colour::Blue.paint("INFO"), msg);
    } else {
        println!("[ INFO ] {}", msg);
    }
}
pub fn api(msg: &str) {
    if config_color() {
        println!("[ {} ] - {}", Colour::Cyan.paint("API"), msg);
    } else {
        println!("[ API ] {}", msg);
    }
}
pub fn warn(msg: &str) {
    if config_color() {
        println!("[ {} ] - {}", Colour::Yellow.paint("WARN"), msg);
    } else {
        println!("[ WARN ] {}", msg);
    }
}