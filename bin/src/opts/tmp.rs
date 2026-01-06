use std::fs;
use std::path::Path;
use std::io;

pub fn copy_dir_all(from: &Path, to: &Path) -> io::Result<()> {
    if !from.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source directory not found"));
    }

    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    // Read the contents of the source directory
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let dest = to.join(entry.file_name());

        if path.is_dir() {
            // Recursively copy subdirectories
            copy_dir_all(&path, &dest)?;
        } else {
            // Copy the file
            fs::copy(&path, &dest)?;
        }
    }

    Ok(())
}

