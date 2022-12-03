use tauri::Manager;

pub fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    manager
        .emit_all("rs2js", format!("rs: {}", message))
        .unwrap();
}
