use futures::prelude::*;
use log::{debug, error, info, log_enabled, Level};
use std::process::Command;
use tokio::prelude::*;
mod settings;
const BUCKET_URL: &str = "asd";
const ROOT_USER: &str = "manish";
const ROOT_PASS: &str = "terminator";
const CONTAINER_NAME: &str = "local-mongo";

async fn app() {
    todo!();
}

fn dump(name: &str, user: &str, pass: &str, root: &settings::Root) {
    let cmd = format!("mongodump  -u {} -p{} --authenticationDatabase=admin --db={} --gzip --archive=dumparchives/{}.archive", root.user, root.pass, name, name );
    let output = Command::new(cmd).output().expect("Failed to dump");
}

fn restore(name: &str, user: &str, pass: &str, root: &settings::Root) {
    let cmd = format!("mongorestore  -u {} -p{} --authenticationDatabase=admin --db={} --gzip --archive=dumparchives/{}.archive", root.user, root.pass, name, name );
    let output = Command::new(cmd).output().expect("Failed to dump");
}

// #[tokio::main]
fn main() {
    env_logger::init();

    let sets = settings::Settings::new().unwrap();

    println!("{:?}", sets);

    for db in sets.dbs.dbs {
        dump(&db.name, &db.user, &db.pass, &sets.root);
    }

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let future = app();

    rt.block_on(future);
}
