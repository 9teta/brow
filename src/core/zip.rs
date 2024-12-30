use std::{fs::File, io::Cursor, path::Path};
use bytes::Bytes;
use zip::ZipArchive;


pub fn extract_file_from_zip_bytes(bytes: &Bytes, location_extracted_file: &str, extracted_file_name: &str, output_dir: &Path) -> Result<(), std::io::Error>{
    log::debug!("Extracting file from zip bytes");
    let output_path = output_dir.join(extracted_file_name);
    // Create a cursor for the Bytes
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;
    log::trace!("Created zip archive from bytes");
    // Locate the file in the archive
    let mut file = archive.by_name(location_extracted_file)?;
    log::trace!("Found extracting file in zip archive");
    
    // Write the file to the output location. Rewrite it if it exists
    let mut output_file = File::options().read(false).write(true).create(true).truncate(false).open(&output_path)?;
    log::trace!("Target file opened to write");
    std::io::copy(&mut file, &mut output_file)?;
    log::debug!("Target file is written");
    Ok(())
}