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

fn main() {
    let pkgs = ftp::DebianFtp::new().list_pkgs("curl").unwrap().filter_by_arch(triples::Arch::Amd64, "curl").to_pkg();

    for pkg in pkgs {
        println!("{:#?}\n, {:?}", pkg.clone(), pkg.extract());
    }

    let app = App::parse();

    match app.subcmd {
        SubCmd::Install(opts) => {
            println!("Installing {}", opts.package);
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
