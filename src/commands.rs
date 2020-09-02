use std::fs;
use std::os::unix::fs::symlink;
use std::path;

pub struct BinSwap {
    pub config_location: path::PathBuf,
    pub archive_location_relative: path::PathBuf,
    pub active_location_relative: path::PathBuf,
}

// API
impl BinSwap {
    pub fn init(self) -> std::io::Result<()> {
        fs::create_dir_all(self.active_dir())?;
        fs::create_dir_all(self.archive_dir())?;
        Ok(())
    }

    pub fn swap(self, name: &str, version: &str) -> std::io::Result<()> {
        // Symlink from active to archive
        match symlink(self.archive_bin(&name, &version), self.active_bin(name)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    pub fn current(self, name: &str) -> std::io::Result<()> {
        // Read the actual file we're symlinked to
        match fs::read_link(&self.active_bin(&name)) {
            Ok(archived_bin) => println!(
                "Currently using version {:?} of binary {:?}",
                archived_bin.parent(),
                name,
            ),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn add(self, name: &str, version: &str, path: &path::PathBuf) -> std::io::Result<()> {
        let archive_bin_dir = self.archive_bin_dir(&name, &version);

        // Create archive directory
        fs::create_dir_all(&archive_bin_dir)?;

        // Copy the binary into the archive path
        match fs::copy(path, self.archive_bin(&name, &version)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        self.swap(&name, &version);

        Ok(())
    }

    pub fn remove(self, name: &str, version: &str) -> std::io::Result<()> {
        Ok(())
    }
}

// Helpers
impl BinSwap {
    fn active_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.active_location_relative.clone())
    }
    fn archive_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.archive_location_relative.clone())
    }
    fn active_bin_dir(&self) -> path::PathBuf {
        self.active_dir()
    }
    fn archive_bin_dir(&self, name: &str, version: &str) -> path::PathBuf {
        self.archive_dir().join(name).join(version)
    }
    fn active_bin(&self, name: &str) -> path::PathBuf {
        self.active_bin_dir().join(&name)
    }
    fn archive_bin(&self, name: &str, version: &str) -> path::PathBuf {
        self.archive_bin_dir(&name, &version).join(&name)
    }
}
