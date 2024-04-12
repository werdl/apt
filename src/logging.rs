use serde::{Serialize, Deserialize};

use std::io::{self, Write};

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

impl Log for Vec<LogEntry> {
    fn write(self) -> Result<(), io::Error> {
        // append to (or create) the log file
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/var/log/apt-alt.log")?;

        // get a list of all log entries, then write them to the file in json format
        let entries = self.into_iter().map(|x| serde_json::to_string(&x).unwrap()).collect::<Vec<String>>();

        for entry in entries {
            writeln!(file, "{}", entry)?;
        }

        Ok(())
    }
}