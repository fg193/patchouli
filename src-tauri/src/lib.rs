#[tauri::command]
async fn search_books(
    request: patchouli_opac_api::SearchRequest,
) -> Result<patchouli_opac_api::SearchResponse, patchouli_opac_api::ApiError> {
    let mut opac = patchouli_opac_api::Opac::new();
    for provider in patchouli_opac_api::providers::all() {
        opac.register(provider);
    }
    opac.search(request).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    tauri::Builder::default()
        .plugin(tauri_plugin_system_ui::init())
        .invoke_handler(tauri::generate_handler![search_books])
        .run(tauri::generate_context!())
        .expect("error while running Patchouli");
}
