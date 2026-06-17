use std::path::Path;
use std::path::PathBuf;

fn gen_main() {}
pub fn compile_file(path: &Path, _output: &Option<PathBuf>) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("file does not exist: {:?}", path));
    }
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(())
}
