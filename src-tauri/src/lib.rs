
#[tauri::command]
fn yap(name: &str) -> String {
    format!("Hello, {}! This is where the ai will start yapping uncontrollably, but still 
    pretty useful information like that one friend. You just need someone there for you!", name)
}
#[tauri::command]
fn inspire(name: &str) -> String {
    format!("Hello, {}! The ai will inspire you to never give up!", name)
}
#[tauri::command]
fn inform(name: &str) -> String {
    format!("Hello, {}! The ai will return your current events here. Curated of course.", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![yap, inspire, inform])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
