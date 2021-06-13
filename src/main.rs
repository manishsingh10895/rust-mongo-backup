use chrono::prelude::*;
use futures::prelude::*;
use log::{debug, error, info, log_enabled, Level};
use std::fs::{self, DirEntry, ReadDir};
use std::process::Command;
use tokio::prelude::*;
mod settings;
mod upload;
use std::fmt;
use structopt::StructOpt;
const BUCKET_URL: &str = "asd";
const ROOT_USER: &str = "manish";
const ROOT_PASS: &str = "terminator";
const CONTAINER_NAME: &str = "local-mongo";

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    //Path of the config file
    config: Option<std::path::PathBuf>,

    #[structopt(short = "op", long = "operation")]
    operation: String,

    #[structopt(short = "a", long = "archive", parse(from_os_str))]
    archives: Option<std::path::PathBuf>,
}

async fn app() {
    todo!();
}

fn get_time() -> String {
    let time: DateTime<Utc> = Utc::now();

    format!("{}", time.clone().format("%d-%b-%Y-%T"))
}

fn dump(name: &str, user: &str, pass: &str, root: &settings::Root) {
    // let cmd = format!("mongodump  -u {} -p{} --authenticationDatabase=admin --db={} --gzip --archive={}{}.archive", root.user, root.pass, name, name, "rosoa" );

    let archive_name = format!("./dumparchives/{}{}.archive", name, get_time());

    println!("----DUMPING  {}-----", archive_name);

    // println!("{}", cmd);
    let output = Command::new("mongodump")
        .args(&[
            "-u",
            root.user.as_str(),
            "-pterminator",
            "--authenticationDatabase",
            "admin",
            &format!("--db={}", &name),
            "--gzip",
            &format!("--archive={}", archive_name),
        ])
        .output();

    match output {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{}", err);
            panic!(err);
        }
    }

    // upload::upload("todos.archive");
}

fn restore(root: &settings::Root, archives: Option<std::path::PathBuf>) {
    let entries: fs::ReadDir = fs::read_dir(archives.unwrap()).unwrap();

    for entry in entries {
        let x: fs::DirEntry = entry.unwrap();
    }

    let output = Command::new("mongorestore")
        .args(&[
            "-u",
            root.user.as_str(),
            format!("-p{}", root.pass),
            "--authenticationDatabase",
            "admin",
            // &format!("--db={}", &name),
            "--gzip",
            &format!("--archive={}", "asd"),
        ])
        .output();

    match output {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{}", err);
            panic!(err);
        }
    };
}

// #[tokio::main]
fn main() {
    env_logger::init();

    let options = Options::from_args();

    let sets = settings::Settings::new(options.config).unwrap();

    println!("{:?}", sets);

    if (!std::path::Path::new("./dumparchives").exists()) {
        std::fs::create_dir_all("./dumparchives").unwrap();
    }

    let op: &str = &options.operation;

    match op {
        "dump" => {
            for db in sets.dbs.dbs {
                dump(&db.name, &db.user, &db.pass, &sets.root);
            }
        }
        "restore" => {
            for db in sets.dbs.dbs {
                restore(&sets.root, options.archives.clone());
            }
        }
        _ => panic!("Error operation"),
    }
}
