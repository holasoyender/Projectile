pub fn banner() {
    println!(r#"
    ____               _           __  _ __
   / __ \_________    (_)__  _____/ /_(_) /__
  / /_/ / ___/ __ \  / / _ \/ ___/ __/ / / _ \
 / ____/ /  / /_/ / / /  __/ /__/ /_/ / /  __/
/_/   /_/   \____/_/ /\___/\___/\__/_/_/\___/
                /___/
Versión: {}. Compilado en: {}.
"#, env!("CARGO_PKG_VERSION"), "Unknown");
}

pub fn ok(msg: &str) {
    println!("[ OK ] {}", msg);
}
pub fn error(msg: &str) {
    println!("[ ERROR ] {}", msg);
}
pub fn info(msg: &str) {
    println!("[ INFO ] {}", msg);
}