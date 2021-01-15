use futures::prelude::*;
use log::{debug, error, info, log_enabled, Level};
use std::process::Command;
use tokio::prelude::*;

const BUCKET_URL: &str = "asd";
const ROOT_USER: &str = "manish";
const ROOT_PASS: &str = "terminator";
const CONTAINER_NAME: &str = "local-mongo";

async fn app() {
    todo!();
}

fn dump() {
    let db = "todos";
    let cmd = format!("mongodump  -u {} -p{} --authenticationDatabase=admin --db={} --gzip --archive=dumparchives/{}.archive", ROOT_USER, ROOT_PASS, db, db );
    let output = Command::new(cmd).output().expect("Failed to dump");
}

fn restore() {
    let db = "todos";
    let cmd = format!("mongorestore  -u {} -p{} --authenticationDatabase=admin --db={} --gzip --archive=dumparchives/{}.archive", ROOT_USER, ROOT_PASS, db, db );
    let output = Command::new(cmd).output().expect("Failed to dump");
}

// #[tokio::main]
fn main() {
    env_logger::init();

    dump();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let future = app();

    rt.block_on(future);
}
