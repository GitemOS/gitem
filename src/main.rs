use alpm::{Alpm, SigLevel, Package, Pkg, Db, AlpmList, Ver};
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, SerializeAs};
use tokio;
use warp::Filter;

/// Simple program to greet a person
#[derive(Parser, Deserialize, Serialize)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    list: Option<ListCommands>
}

#[derive(Subcommand, Debug, Deserialize, Serialize)]
enum ListCommands {
    List {
        #[clap(short, long, action)]
        all: bool,
    }
}

#[serde_as]
#[derive(Serialize)]
#[serde(remote = "Package")]
struct PackageDef<'a> {
    #[serde(getter = "Pkg::name")]
    name: &'a str,
    #[serde(getter = "Pkg::filename")]
    filename: &'a str,
    // #[serde(serialize_with = "into")]
    // #[serde(getter = "Pkg::version")]
    // version: &'a str,
    #[serde(getter = "Pkg::desc")]
    desc: Option<&'a str>,
    #[serde(getter = "Pkg::arch")]
    arch: Option<&'a str>,
}

impl SerializeAs<Package<'_>> for PackageDef<'_> {
    fn serialize_as<S>(value: &Package, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {  
        PackageDef::serialize(value, serializer)
    }
}

#[serde_as] // https://docs.rs/serde_with/1.9.2/serde_with/guide/serde_as/index.html
#[derive(Serialize)]
struct Packages<'a> {
    #[serde_as(as = "Vec<PackageDef>")]
    packages: Vec<Package<'a>>,

}


#[tokio::main]
async fn main() {
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

        let cors = warp::cors().allow_any_origin();

        // https://github.com/seanmonstar/warp/blob/master/examples/body.rs
        // GET /hello/warp => 200 OK with body "Hello, warp!"
        let hello = warp::get()
            .and(warp::path("hello"))
            .and(warp::path::param::<String>())
            .map(|name| {
                let handle = Alpm::new("/", "/var/lib/pacman/").unwrap();

                handle
                    .register_syncdb("community", SigLevel::USE_DEFAULT)
                    .unwrap();
                
                let local_db = handle.localdb();
                let vec_packages: Vec<Package> = local_db.pkgs().iter().collect();
                let packages = Packages { packages: vec_packages };
                println!("Aha, {}!", &name);
                warp::reply::json(&packages) // local_packages
            })
            .with(cors);

        // warp::path!("hello" / String / i32)
        //     .map(|name, num| format!("Hello, {}, #{}!", name, num));


        warp::serve(hello)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }
    


}
