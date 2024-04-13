use ar::Archive;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;

use tar::Archive as TarArchive;

use crate::defs::Package;


fn recursive_list_files(dir: &str) -> Vec<String> {
    let mut files = vec![];

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut recursive_list_files(path.to_str().unwrap()));
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }

    files
}

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
        let mut files = recursive_list_files("/tmp/apt-alt/data");

        // now trim the /tmp/apt-alt/data prefix
        files = files.iter().map(|x| x.trim_start_matches("/tmp/apt-alt/data").to_string()).collect();


        // now we have all the files, copy them to the real filesystem
        for file in files.iter() {
            
            let abs_path = format!("{}", file);

            println!("Copying {} to {}", file, abs_path);

            let mut file = File::open(format!("/tmp/apt-alt/data{}", file)).unwrap();

            // make sure that the parent directory exists
            std::fs::create_dir_all(std::path::Path::new(&abs_path).parent().unwrap()).unwrap();

            let mut new_file = File::create(format!("{}", abs_path)).unwrap();

            io::copy(&mut file, &mut new_file).unwrap();
        }

        // now, for each file, if it's path is in $PATH, make it executable
        for file in files.iter() {
            if std::env::var("PATH").unwrap().split(":").any(|x| file.starts_with(x)) {
                std::fs::set_permissions(file, std::fs::Permissions::from_mode(0o755)).unwrap();
            }
        }

        // now remove the /tmp/apt-alt directory
        std::fs::remove_dir_all("/tmp/apt-alt").unwrap();

        Ok(LogEntry::new(self.name, files))
        
       
    }
}
