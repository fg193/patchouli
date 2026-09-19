use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<SystemUi<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("io.github.fg193.patchouli.systemui", "SystemUiPlugin")?;
    Ok(SystemUi(handle))
}

/// Access to the system-ui APIs.
pub struct SystemUi<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> SystemUi<R> {
    pub fn set_status_bar_style(&self, payload: StatusBarStyle) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("setStatusBarStyle", payload)
            .map_err(Into::into)
    }
}
