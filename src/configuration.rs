use crate::evm;
use anyhow::Result;
use std::path;

pub struct EvmConfiguration {
    pub profile_location: path::PathBuf,
    pub config_location: path::PathBuf,
    pub archive_location_relative: path::PathBuf,
    pub active_location_relative: path::PathBuf,
}

impl EvmConfiguration {
    pub fn init(self) -> Result<()> {
        let res = evm::initialise(&self);
        println!("Successfully initialised evm. Please run the following command to use evm in this session:");
        println!("source {:?}", self.profile_location);

        res
    }

    pub fn swap(self, name: &String, version: &String) -> Result<()> {
        let res = evm::swap_to_version(&self, name, version);
        println!("Activated {} {}", name, version);

        res
    }

    pub fn list(self, name: Option<String>) -> Result<()> {
        match name {
            Some(name) => {
                let active = evm::get_active_version(&self, &name)?;
                let versions = evm::list_versions(&self, &name)?;
                for entry in versions.iter() {
                    let file_name = entry.file_name().unwrap().to_string_lossy();
                    if active.eq(&file_name) {
                        print!("->");
                    } else {
                        print!("  ");
                    }
                    println!(" {}", file_name);
                }
            }
            None => {
                let versions = evm::list_binaries(&self)?;
                for entry in versions.iter() {
                    let folder_name = entry.file_name().unwrap().to_string_lossy();
                    println!(" {}", folder_name);
                }
            }
        }

        Ok(())
    }

    pub fn active(self, name: &String) -> Result<()> {
        println!("{}", evm::get_active_version(&self, name)?);
        Ok(())
    }

    pub fn add(self, name: &String, version: &String, path: &path::PathBuf) -> Result<()> {
        let res = evm::add_bin_version(&self, name, version, path);
        println!("Added {} {}", name, version);

        res
    }

    pub fn remove(self, name: &String, version: &Option<String>) -> Result<()> {
        let res = evm::remove_bin_version(&self, name, version);
        match res {
            Ok(_) => {
                match version {
                    Some(version) => println!("Removed {} {}", name, version),
                    None => println!("Removed {}", name),
                };
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    }
}

// Helpers
impl EvmConfiguration {
    pub fn active_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.active_location_relative.clone())
    }
    pub fn active_bin(&self, name: &String) -> path::PathBuf {
        self.active_dir().join(&name)
    }
    pub fn archive_dir(&self) -> path::PathBuf {
        self.config_location
            .join(self.archive_location_relative.clone())
    }
    pub fn archive_bin(&self, name: &String) -> path::PathBuf {
        self.archive_dir().join(&name)
    }
    pub fn archive_bin_ver_dir(&self, name: &String, version: &String) -> path::PathBuf {
        self.archive_dir().join(&name).join(&version)
    }
    pub fn archive_bin_ver(&self, name: &String, version: &String) -> path::PathBuf {
        self.archive_bin_ver_dir(&name, &version).join(&name)
    }
}
