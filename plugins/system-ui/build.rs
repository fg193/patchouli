const COMMANDS: &[&str] = &["set_status_bar_style"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
