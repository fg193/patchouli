use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<SystemUi<R>> {
    Ok(SystemUi(app.clone()))
}

/// Access to the system-ui APIs.
pub struct SystemUi<R: Runtime>(AppHandle<R>);

impl<R: Runtime> SystemUi<R> {
    pub fn set_status_bar_style(&self, _payload: StatusBarStyle) -> crate::Result<()> {
        Ok(())
    }
}
