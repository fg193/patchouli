use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(not(target_os = "android"))]
mod desktop;
#[cfg(target_os = "android")]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(not(target_os = "android"))]
use desktop::SystemUi;
#[cfg(target_os = "android")]
use mobile::SystemUi;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the system-ui APIs.
pub trait SystemUiExt<R: Runtime> {
    fn system_ui(&self) -> &SystemUi<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SystemUiExt<R> for T {
    fn system_ui(&self) -> &SystemUi<R> {
        self.state::<SystemUi<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("system-ui")
        .invoke_handler(tauri::generate_handler![commands::set_status_bar_style])
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let system_ui = mobile::init(app, api)?;
            #[cfg(not(target_os = "android"))]
            let system_ui = desktop::init(app, api)?;
            app.manage(system_ui);
            Ok(())
        })
        .build()
}
