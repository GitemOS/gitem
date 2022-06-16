use alpm::{Alpm, SigLevel};
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    list: Option<ListCommands>
}

#[derive(Subcommand, Debug)]
enum ListCommands {
    List {
        #[clap(short, long, action)]
        all: bool,
    }
}

fn main() {
    let args = Args::parse();
    if let Some(list) = args.list {
        println!("His name is {:?}", list);
    }
    else {
            // let cwd = std::env::current_dir().unwrap();
        // println!("The cwd path is: {:?}", cwd.as_path());
        let handle = Alpm::new("/", "/var/lib/pacman/").unwrap();

        handle
            .register_syncdb("community", SigLevel::USE_DEFAULT)
            .unwrap();
        
        // for db in handle.syncdbs() {
        //     println!("The db name is: {}", db.name());
        //     let package_list = db.pkgs();
        //     println!("{:?}", package_list);
        // }
        let local_db = handle.localdb();
        let local_packages = local_db.pkgs();
        for pkg in local_packages {
            println!("Package Name: {} Package Version: {} Arch: {}", pkg.name(), pkg.version(), pkg.arch().unwrap());
        }
        // println!("The packages: {:?}", local_packages.len());
    }
    
}
