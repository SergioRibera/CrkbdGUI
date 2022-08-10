fn main() {
    if !cfg!(test) {
        tauri_build::build()
    }
}
