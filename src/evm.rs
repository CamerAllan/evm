use crate::commands::EvmConfiguration;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Read, Write};
use std::os::unix::fs::symlink;
use std::path;

pub fn initialise(config: &EvmConfiguration) -> std::io::Result<()> {
    let active_dir = config.active_dir();

    // Prepend the active directory to PATH
    let mut profile = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(&config.profile_location)
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
    fs::create_dir_all(config.active_dir())?;
    fs::create_dir_all(config.archive_dir())?;
    Ok(())
}

pub fn swap_to_version(
    config: &EvmConfiguration,
    name: &str,
    version: &str,
) -> std::io::Result<()> {
    let active_bin = config.active_bin(name);

    // Remove existing symlink
    match fs::remove_file(&active_bin) {
        Ok(_) => (),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => (), // First time activating this name
            err => panic!("Error removing existing symlink"),
        },
    }
    // Symlink from active to archive
    match symlink(config.archive_bin_ver(&name, &version), &active_bin) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }
    Ok(())
}

pub fn list_versions(
    config: &EvmConfiguration,
    name: &str,
) -> std::io::Result<std::vec::Vec<path::PathBuf>> {
    let mut entries = fs::read_dir(config.archive_bin(&name))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()?;
    entries.sort();

    Ok(entries)
}

pub fn get_active_version(config: &EvmConfiguration, name: &str) -> std::io::Result<String> {
    // Read the actual file we're symlinked to
    match fs::read_link(config.active_bin(&name)) {
        Ok(archive_bin) => Ok(archive_bin
            .parent() // Ewwww
            .unwrap() // Ewwww
            .file_name() // Ewwww
            .unwrap() // Ewwww
            .to_str() // Ewwww
            .unwrap()
            .to_string()), // Ewwww please fix
        Err(err) => return Err(err),
    }
}

pub fn add_bin_version(
    config: &EvmConfiguration,
    name: &str,
    version: &str,
    path: &path::PathBuf,
) -> std::io::Result<()> {
    let archive_bin_dir = config.archive_bin_ver_dir(&name, &version);

    // Create archive directory
    fs::create_dir_all(&archive_bin_dir)?;

    // Copy the binary into the archive path
    match fs::copy(path, &config.archive_bin_ver(&name, &version)) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    swap_to_version(&config, &name, &version)?;

    Ok(())
}

pub fn remove_bin_version(
    config: &EvmConfiguration,
    name: &str,
    version: &Option<String>,
) -> std::io::Result<()> {
    let active = get_active_version(&config, &name).unwrap();
    let active_bin = config.active_bin(&name);
    let dir_to_remove: path::PathBuf;

    match version {
        Some(version) => {
            dir_to_remove = config.archive_bin_ver_dir(name, version);

            if version == &active {
                // Remove symlink only if we're removing currenlty activated version
                if let Err(err) = fs::remove_file(active_bin) {
                    return Err(err);
                }
            }
        }
        None => {
            // Remove all versions of this binary
            dir_to_remove = config.archive_bin(&name);
            // Remove symlink
            if let Err(err) = fs::remove_file(active_bin) {
                return Err(err);
            }
        }
    }

    // Remove only this version of this binary
    if let Err(err) = fs::remove_dir_all(&dir_to_remove) {
        return Err(err);
    }

    Ok(())
}
