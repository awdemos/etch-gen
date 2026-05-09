use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct CrateWriter {
    output_dir: PathBuf,
}

impl CrateWriter {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_path_buf(),
        }
    }

    pub fn write(
        &self,
        design_name: &str,
        manifest: String,
        files: Vec<(PathBuf, String)>,
    ) -> io::Result<PathBuf> {
        let crate_dir = self.output_dir.join(design_name);
        let src_dir = crate_dir.join("src");

        fs::create_dir_all(&src_dir)?;

        fs::write(crate_dir.join("Cargo.toml"), manifest)?;

        for (relative_path, content) in files {
            let full_path = crate_dir.join(&relative_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&full_path, content)?;
        }

        Ok(crate_dir)
    }
}
