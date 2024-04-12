use ar::Archive;
use std::fs::File;
use std::io::{self, Read, Write};

use tar::Archive as TarArchive;

use crate::defs::Package;

use crate::logging::LogEntry;

pub trait Extract {
    fn extract(self) -> Result<LogEntry, io::Error>; // <path_to_extracted_file>, <error>
}

impl Extract for Package {
    fn extract(self) -> Result<LogEntry, io::Error> {
        // create the 3 needed directories: /tmp/apt-alt, /tmp/apt-alt/data, /tmp/apt-alt/control
        std::fs::create_dir_all("/tmp/apt-alt/data").unwrap();
        std::fs::create_dir_all("/tmp/apt-alt/control").unwrap();


        // get the contents of the deb file from the ftp server
        let contents = reqwest::blocking::get(&self.deb_link).unwrap().bytes().unwrap();

        let mut archive = Archive::new(contents.as_ref());
        
        // extract data.tar.xz and control.tar.xz to /tmp/apt-alt
        while let Some(entry_result) = archive.next_entry() {
            let mut entry = entry_result.unwrap();
            // Create a new file with the same name as the archive entry:

            let path = std::str::from_utf8(entry.header().identifier()).unwrap_or_default();

            if path.ends_with("data.tar.xz") {
                let mut file = File::create(
                    format!("/tmp/apt-alt/data.tar.xz"),
                ).unwrap();
                // The Entry object also acts as an io::Read, so we can easily copy the
                // contents of the archive entry into the file:
                io::copy(&mut entry, &mut file).unwrap();
            } else if path.ends_with("control.tar.xz") {
                let mut file = File::create(
                    format!("/tmp/apt-alt/control.tar.xz"),
                ).unwrap();
                // The Entry object also acts as an io::Read, so we can easily copy the
                // contents of the archive entry into the file:
                io::copy(&mut entry, &mut file).unwrap();
            }
        }

        // extract data.tar.xz and control.tar.xz
        let data_tar = File::open("/tmp/apt-alt/data.tar.xz").unwrap();
        let data_tar = xz2::read::XzDecoder::new(data_tar);
        let mut data_tar = TarArchive::new(data_tar);

        data_tar.unpack("/tmp/apt-alt/data").unwrap();

        let control_tar = File::open("/tmp/apt-alt/control.tar.xz").unwrap();
        let control_tar = xz2::read::XzDecoder::new(control_tar);
        let mut control_tar = TarArchive::new(control_tar);

        control_tar.unpack("/tmp/apt-alt/control").unwrap();

        // now, list all files in /tmp/apt-alt/data, including anything in subdirectories (but not the subdirectories themselves)
        let mut files = vec![];

        for entry in std::fs::read_dir("/tmp/apt-alt/data").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let path = path.to_str().unwrap();
            files.push(path.to_string());
        }

        Ok(LogEntry::new(self.name, files))
        
       
    }
}
