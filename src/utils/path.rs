use std::path::Path;

pub fn path_exists(path: &Path) -> bool {
    path.exists()
}
