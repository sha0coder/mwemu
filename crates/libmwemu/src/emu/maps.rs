use std::{fs, io, path::Path};

use crate::emu::Emu;
use crate::maps::mem64::Permission;

impl Emu {
    /// For simulating a windows process space, select the folder with maps32 or maps64 depending upon the arch, do this before loading the binary.
    pub fn set_maps_folder(&mut self, folder: &str) {
        //let mut f = folder.to_string();
        //f.push('/');
        self.cfg.maps_folder = folder.to_string();

        // Check if maps folder exists and contains essential files
        if !self.maps_folder_is_valid(folder) {
            log::info!(
                "Maps folder '{}' not found or incomplete, attempting to download...",
                folder
            );
            if let Err(e) = self.download_and_extract_maps(folder) {
                log::error!("Failed to download maps folder '{}': {}", folder, e);
                panic!("Cannot proceed without maps folder. Please download manually or check your internet connection.");
            }
        }
    }

    /// Check if maps folder exists and contains essential files
    fn maps_folder_is_valid(&self, folder: &str) -> bool {
        let folder_path = Path::new(folder);
        if !folder_path.exists() {
            return false;
        }

        // Check for essential files based on architecture
        let essential_files = if folder.contains("32") {
            vec!["ntdll.dll", "kernel32.dll", "banzai.csv"]
        } else {
            vec!["ntdll.dll", "kernel32.dll"]
        };

        for file in essential_files {
            let file_path = folder_path.join(file);
            if !file_path.exists() {
                log::info!(
                    "Essential file '{}' missing from maps folder",
                    file_path.display()
                );
                return false;
            }
        }

        true
    }

    /// Download and extract maps folder from specific URL
    fn download_and_extract_maps(&self, folder: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = match folder {
            "maps32" | "maps32/" | "maps/maps32" | "maps/maps32/" => {
                "https://github.com/sha0coder/mwemu/releases/download/maps/maps32.zip"
            }
            "maps64" | "maps64/" | "maps/maps64" | "maps/maps64/" => {
                "https://github.com/sha0coder/mwemu/releases/download/maps/maps64.zip"
            }
            _ => return Err(format!("Unknown maps folder: {}", folder).into()),
        };

        log::info!(
            "Downloading {} from GitHub releases... (this may take a moment)",
            folder
        );

        // Download the file using ureq
        // Note: To reduce TLS verbosity, set RUST_LOG=info instead of debug
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(30))
            .call()?;

        if response.status() != 200 {
            return Err(format!("Failed to download: HTTP {}", response.status()).into());
        }

        let mut bytes = Vec::new();
        response.into_reader().read_to_end(&mut bytes)?;
        log::info!("Downloaded {} MB", bytes.len() / 1024 / 1024);

        // Extract the zip file
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor)?;

        log::info!("Extracting {} files...", archive.len());

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = Path::new(file.name());

            if file.name().ends_with('/') {
                // Create directory
                fs::create_dir_all(&outpath)?;
            } else {
                // Create parent directories if they don't exist
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }

                // Extract file
                let mut outfile = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }

        log::info!("Successfully extracted maps folder: {}", folder);
        Ok(())
    }

    /// Get the base address of the code, if code map doesn't exist yet will return None.
    pub fn get_base_addr(&self) -> Option<u64> {
        //TODO: fix this, now there is no code map.
        let map = match self.maps.get_map_by_name("code") {
            Some(m) => m,
            None => return None,
        };

        Some(map.get_base())
    }

    /// From a file-path this returns the filename with no path and no extension.
    pub fn filename_to_mapname(&self, filename: &str) -> String {
        filename
            .split('/')
            .last()
            .map(|x| x.split('.'))
            .and_then(|x| x.peekable().next())
            .unwrap()
            .to_string()
    }

    /// Remove from the memory the map name provided.
    pub fn free(&mut self, name: &str) {
        self.maps.free(name);
    }

    /// This find an empty space on the memory of selected size
    /// and also creates a map there.
    pub fn alloc(&mut self, name: &str, size: u64, permission: Permission) -> u64 {
        let addr = match self.maps.alloc(size) {
            Some(a) => a,
            None => {
                log::info!("low memory");
                return 0;
            }
        };
        self.maps
            .create_map(name, addr, size, permission)
            .expect("cannot create map from alloc api");
        addr
    }
}
