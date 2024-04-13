use serde::{Serialize, Deserialize};

use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    modified_files: Vec<String>,
    package: String,
}

impl LogEntry {
    pub fn new(package: String, modified_files: Vec<String>) -> LogEntry {
        LogEntry {
            modified_files: modified_files,
            package: package,
        }
    }
}

pub trait Log {
    fn write(self) -> Result<(), io::Error>;
}

// when appending to the file, first read in the existing JSON data, then append to the JSONic list found (or create the file if not)

impl Log for LogEntry {
    fn write(self) -> Result<(), io::Error> 
    {
        // ensure the log file exists
        std::fs::create_dir_all("/opt/log/apt-alt")?;


        // first read from file
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/opt/log/apt-alt/log.json")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // parse the JSON
        let mut log: Result<Vec<LogEntry>, serde_json::Error> = serde_json::from_str(&contents);

        // if the file is empty, create a new Vec
        let mut log_vec: Vec<LogEntry> = match log {
            Ok(v) => v,
            Err(_) => Vec::new(),
        };

        // append the new entry
        log_vec.push(self);

        println!("{:?}", log_vec);

        // write the new JSON to the file
        let new_contents = serde_json::to_string(&log_vec)?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("/opt/log/apt-alt/log.json")?;

        file.write_all(new_contents.as_bytes())?;

        Ok(())
    }
}

impl Log for Vec<LogEntry> {
    fn write(self) -> Result<(), io::Error> {
        for entry in self {
            entry.write()?;
        }

        Ok(())
    }
}