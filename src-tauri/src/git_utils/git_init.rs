use git2::Repository;

#[tauri::command]
pub fn init(path: &str) {
    let _repo = match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to init {}", e),
    };
}