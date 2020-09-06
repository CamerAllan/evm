use crate::evm;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Read, Write};
use std::os::unix::fs::symlink;
use std::path;

pub struct EvmConfig {
    pub profile_location: path::PathBuf,
    pub config_location: path::PathBuf,
    pub archive_location_relative: path::PathBuf,
    pub active_location_relative: path::PathBuf,
}

// API
impl EvmConfig {
    pub fn init(self) -> std::io::Result<()> {
        let active_dir = &self.active_dir();

        // Prepend the active directory to PATH
        let mut profile = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&self.profile_location)
            .unwrap();

        let export = format!("export PATH={:?}:$PATH", &active_dir);

        let mut profile_contents = String::new();
        profile
            .read_to_string(&mut profile_contents)
            .expect("Can't read profile");

        if !profile_contents.contains(&export) {
            // Only add export once
            if let Err(err) = writeln!(profile, "{}", &export) {
                eprintln!("Couldn't write to file '{:?}': {}", &active_dir, err);
            }
        }

        // Create base directories in config
        fs::create_dir_all(self.active_dir())?;
        fs::create_dir_all(self.archive_dir())?;
        Ok(())
    }

    pub fn swap(self, name: &str, version: &str) -> std::io::Result<()> {
        let active_bin = self.active_bin(name);

        // Remove existing symlink
        match fs::remove_file(&active_bin) {
            Ok(_) => (),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => (), // First time activating this name
                _ => panic!("Error removing existing symlink"),
            },
        }
        // Symlink from active to archive
        match symlink(self.archive_bin_ver(&name, &version), &active_bin) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        Ok(())
    }

    pub fn list(self, name: &str) -> std::io::Result<()> {
        let mut entries = fs::read_dir(&self.archive_bin(&name))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, Error>>()?;
        entries.sort();

        for entry in entries.iter() {
            println!("{:?}", entry.file_name().unwrap())
        }

        Ok(())
    }

    pub fn active(self, name: &str) -> std::io::Result<()> {
        // Read the actual file we're symlinked to
        match fs::read_link(&self.active_bin(&name)) {
            Ok(archive_bin) => println!(
                "Active version of {:?} is {:?}",
                name,
                archive_bin.parent().unwrap().file_name().unwrap(),
            ),
            Err(err) => return Err(err),
        }

        Ok(())
    }

    pub fn add(self, name: &str, version: &str, path: &path::PathBuf) -> std::io::Result<()> {
        let archive_bin_dir = self.archive_bin_ver_dir(&name, &version);

        // Create archive directory
        fs::create_dir_all(&archive_bin_dir)?;

        // Copy the binary into the archive path
        match fs::copy(path, self.archive_bin_ver(&name, &version)) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        self.swap(&name, &version)?;

        Ok(())
    }

    pub fn remove(self, name: &str, version: &Option<String>) -> std::io::Result<()> {
        // TODO: Refactor out command implementation, then call active() to see if we need to remove the symlink
        // For now we just leave the symlink

        let dir_to_remove: path::PathBuf;

        match version {
            Some(version) => {
                dir_to_remove = self.archive_bin_ver_dir(name, version);
            }
            None => {
                // Remove all versions of this binary
                dir_to_remove = self.archive_bin(name);
            }
        }
        // Remove only this version of this binary
        if let Err(err) = fs::remove_dir(&dir_to_remove) {
            return Err(err);
        }

        Ok(())
    }
}

// Helpers
impl EvmConfig {
    fn active_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.active_location_relative.clone())
    }
    fn active_bin(&self, name: &str) -> path::PathBuf {
        self.active_dir().join(&name)
    }
    fn archive_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.archive_location_relative.clone())
    }
    fn archive_bin(&self, name: &str) -> path::PathBuf {
        self.archive_dir().join(&name)
    }
    fn archive_bin_ver_dir(&self, name: &str, version: &str) -> path::PathBuf {
        self.archive_dir().join(&name).join(&version)
    }
    fn archive_bin_ver(&self, name: &str, version: &str) -> path::PathBuf {
        self.archive_bin_ver_dir(&name, &version).join(&name)
    }
}
