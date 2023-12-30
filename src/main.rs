#[macro_use]
extern crate rocket;
use std::io;

use rocket::tokio::{
    task::spawn_blocking,
    time::{sleep, Duration},
};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/parvez")]
fn subindex() -> &'static str {
    "This page created by the owner, Parvez Ali"
}
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}
#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, subindex, delay, blocking_task])
        .launch()
        .await?;

    Ok(())
}
