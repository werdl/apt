mod triples;
mod ftp;
mod defs;
mod extracter;
mod logging;

use std::io::Read;

use defs::Filter;
use extracter::Extract;

use clap::{
    Parser,
    Subcommand
};
use logging::Log;

#[derive(Parser)]
struct Opts {
    package: String,
}

#[derive(Subcommand)]
enum SubCmd {
    #[clap(name = "install")]
    Install(Opts),
    #[clap(name = "remove")]
    Remove(Opts),
    #[clap(name = "update")]
    Update,
    #[clap(name = "upgrade")]
    Upgrade,
    #[clap(name = "list")]
    List,
}

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

fn install(package: String, arch: triples::Arch) {
    let mut ftp = ftp::DebianFtp::new();
    let packages = ftp.list_pkgs(&package).unwrap();

    let packages =  packages.filter_by_arch(arch, &package).to_pkg();

    // select the package with the highest version
    let package = packages.iter().max().unwrap().to_owned();

    // now, extract the package
    let log = package.extract().unwrap();

    // write the log
    log.write().unwrap();
}

fn remove(package: String) -> Result<(), std::io::Error> {

    // make sure the path exists
    std::fs::create_dir_all("/opt/log/apt-alt")?;

    // first, read the log file
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/opt/log/apt-alt/log.json").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // parse the JSON
    let mut log: Vec<logging::LogEntry> = serde_json::from_str(&contents)?;

    // now, find the package with the latest version
    let packages = log.iter().filter(|x| x.package == package);

    let latest = packages.max_by(|x, y| x.package_version.partial_cmp(&y.package_version).unwrap()).unwrap();

    // now, find all the files that were installed with this package, and remove them


    for file in &latest.modified_files {
        match std::fs::remove_file(file) {
            Ok(_) => {},
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {},
                    _ => {
                        return Err(e);
                    }
                }
            }
        }
    }

    // now, check each directory of the files, and remove them if they are empty
    for file in &latest.modified_files {
        let path = std::path::Path::new(file);
        let parent = path.parent().expect("Could not get parent directory");

        let read_dir = std::fs::read_dir(parent);

        if read_dir.is_err() {
            continue; // probably a directory already removed
        }

        if read_dir.unwrap().count() == 0 {
            match std::fs::remove_dir(parent) {
                Ok(_) => {},
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => {},
                        _ => {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let app = App::parse();

    match app.subcmd {
        SubCmd::Install(opts) => {
            println!("Installing {}", opts.package);
            install(
                opts.package,
                triples::Arch::Amd64
            )
        },
        SubCmd::Remove(opts) => {
            println!("Removing {}", opts.package);
            remove(opts.package).unwrap();
        },
        SubCmd::Update => {
            println!("Updating");
        },
        SubCmd::Upgrade => {
            println!("Upgrading");
        },
        SubCmd::List => {
            println!("Listing");
        }
    }
}
