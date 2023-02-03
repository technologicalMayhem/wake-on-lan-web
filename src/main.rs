use std::{env, io, process::exit};

#[macro_use]
extern crate lazy_static;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, main};
use async_process::Command;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Wake is running.

                             Use the /wake endpoint to wake the machine up.
                             Use the /shutdown endpoint to shut the machine down.
                             Use the /state endpoint to geth the current state of the machine.")
}

#[get("/wake")]
async fn wake() -> impl Responder {
    let out = match Command::new("wakeonlan")
        .arg(&CONFIG.machine_mac)
        .output()
        .await
    {
        Ok(out) => out,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    if out.status.success() {
        HttpResponse::Ok().body("Success!")
    } else {
        HttpResponse::InternalServerError().body(out.stderr)
    }
}

#[get("/shutdown")]
async fn shutdown() -> impl Responder {
    let destination = format!("{}@{}", CONFIG.username, CONFIG.machine_ip);
    let out = match Command::new("ssh")
        .arg(destination)
        .arg("shutdown now")
        .output()
        .await
    {
        Ok(out) => out,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    if out.status.success() {
        HttpResponse::Ok().body("Success!")
    } else {
        HttpResponse::InternalServerError().body(out.stderr)
    }
}

#[get("/state")]
async fn state() -> impl Responder {
    println!("State");
    let result = match Command::new("ping")
        .arg("-c 1")
        .arg("-w 1")
        .arg(&CONFIG.machine_ip)
        .output()
        .await
    {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if ! result.status.success() {
        return HttpResponse::Ok().body("Down");
    }

    let address = format!("{}:{}/guacamole", CONFIG.machine_ip, CONFIG.guacamole_port);
    let result = match Command::new("wget")
        .arg("-t 1")
        .arg("-T 1")
        .arg("-O-")
        .arg(address)
        .output()
        .await
    {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if result.status.success() {
        return HttpResponse::Ok().body("Ready");
    }
    HttpResponse::Ok().body("Starting or shutting down")
}

struct Config {
    wake_port: u16,
    machine_mac: String,
    machine_ip: String,
    username: String,
    guacamole_port: String
}

fn get_config() -> Config {
    Config {
        wake_port: match get_value("WAKE_PORT").parse() {
            Ok(port) => port,
            Err(_) => {eprintln!("Enviroment variable \'WAKE_PORT\' is not a valid port number.");
            exit(1);},
        },
        machine_mac: get_value("MACHINE_MAC"),
        machine_ip: get_value("MACHINE_IP"),
        username: get_value("USERNAME"),
        guacamole_port: get_value("GUACAMOLE_PORT"),
    }
}

fn get_value(key: &str) -> String {
    match env::var(key) {
        Ok(var) => var,
        Err(_) => {
            eprintln!("Enviroment variable \'{key}\' not set.");
            exit(1);
        }
    }
}
lazy_static! {
    static ref CONFIG: Config = get_config();
}

#[main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(wake)
            .service(shutdown)
            .service(state)
    })
    .bind(("127.0.0.1", CONFIG.wake_port))?
    .run()
    .await
}
