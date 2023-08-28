use super::BuildContext;
use crate::{BCError, StdIOErrorExt};
use flate2::read::GzDecoder;
use leaf::download;
use std::{
    fs::File,
    io::{Read, Seek, Write},
};
use tar::Archive;
use xz::read::XzDecoder;

impl<'a> BuildContext<'a> {
    /// Prepares all the source files for operation
    pub fn prepare_sources(&mut self) -> Result<(), BCError> {
        // Check if there is a source file
        if let Some(url) = &self.pkgbuild.source {
            self.prepare_main_source(&url)?;
        }
        Ok(())
    }

    /// Fetches the main source file and extracts it, if possible
    fn prepare_main_source(&mut self, source_url: &str) -> Result<(), BCError> {
        let url = source_url
            .replace("$PKG_NAME", &self.pkgbuild.name)
            .replace("$PKG_VERSION", &self.pkgbuild.version);

        // Parse out the file name
        let name = &url
            .split("/")
            .last()
            .expect("File name of source file")
            .to_string();

        // Construct a destination path
        let dst_path = self
            .config
            .get_buildroot_build_dir(&self.pkgbuild)
            .join(&name);

        info!(
            "Fetching source from {} to {}",
            url,
            dst_path.to_string_lossy()
        );

        // Open the destination file handle
        let mut source_file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(&dst_path)
            .err_prepend(&format!(
                "Open main source file handle at {}",
                dst_path.to_string_lossy()
            ))?;

        // Download the file using the leaf handler for that
        leaf::error::LErrorExt::err_prepend(
            download::download(
                &url,
                &format!("Downloading source file {}", &name),
                true,
                |data| {
                    source_file.write(data).expect("When writing out file");
                    true
                },
            ),
            &format!("When fetching source from {}", url),
        )?;

        // Seek to begin, read magic bytes and seek to start
        source_file
            .seek(std::io::SeekFrom::Start(0))
            .err_prepend("When seeking to start of source file")?;
        let mut buf: [u8; 8] = [0; 8];
        source_file
            .read_exact(&mut buf)
            .err_prepend("When reading magic bytes of source file")?;
        source_file
            .seek(std::io::SeekFrom::Start(0))
            .err_prepend("When seeking to start of source file")?;

        // Automatically extract XZ, GZ, ZIP archives
        if infer::archive::is_xz(&buf) {
            info!("Source is a XZ archive, extracting...");
            let tar = XzDecoder::new(source_file);

            let mut archive = Archive::new(tar);
            archive.set_overwrite(true);
            archive.unpack(&self.config.get_buildroot_build_dir(&self.pkgbuild))?;
        } else if infer::archive::is_gz(&buf) {
            info!("Source is a GZ archive, extracting...");
            let tar = GzDecoder::new(source_file);

            let mut archive = Archive::new(tar);
            archive.set_overwrite(true);
            archive.unpack(&self.config.get_buildroot_build_dir(&self.pkgbuild))?;
        } else if infer::archive::is_zip(&buf) {
            info!("Source is a ZIP archive, extracting...");

            let mut zip = zip::ZipArchive::new(source_file)?;
            zip.extract(&self.config.get_buildroot_build_dir(&self.pkgbuild))?;
        }

        Ok(())
    }
}
