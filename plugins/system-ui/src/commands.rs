use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::SystemUiExt;

#[command]
pub(crate) async fn set_status_bar_style<R: Runtime>(
    app: AppHandle<R>,
    payload: StatusBarStyle,
) -> Result<()> {
    app.system_ui().set_status_bar_style(payload)
}
