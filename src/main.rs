mod triples;
mod ftp;
mod defs;
mod extracter;
mod logging;

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
