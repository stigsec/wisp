use clap::{Arg, Command};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use tiny_http::{Server, Response, Method};

fn main() {
    let matches = Command::new("wisp")
        .about("Tiny Rust HTTP file server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .num_args(1)
                .value_name("PORT")
                .help("Port to bind to (default 8000)"),
        )
        .get_matches();

    let port = matches
        .get_one::<String>("port")
        .map(|s| s.as_str())
        .unwrap_or("8000");

    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).expect("Could not bind to address");

    println!("Serving files on http://{}/", addr);

    for request in server.incoming_requests() {
        let method = request.method();
        let url = request.url().trim_start_matches('/');

        let remote_addr = request.remote_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string());
        println!("{} {} from {}", method, request.url(), remote_addr);

        match method {
            Method::Get => {
                let path = Path::new(url);
                if path.exists() && path.is_file() {
                    match File::open(path) {
                        Ok(mut file) => {
                            let mut buf = Vec::new();
                            if let Err(_) = file.read_to_end(&mut buf) {
                                let resp = Response::from_string("500 Internal Server Error")
                                    .with_status_code(500);
                                let _ = request.respond(resp);
                            } else {
                                let resp = Response::from_data(buf);
                                let _ = request.respond(resp);
                            }
                        }
                        Err(_) => {
                            let resp = Response::from_string("403 Forbidden")
                                .with_status_code(403);
                            let _ = request.respond(resp);
                        }
                    }
                } else {
                    let resp = Response::from_string("404 Not Found").with_status_code(404);
                    let _ = request.respond(resp);
                }
            }

            Method::Post => {
                if url.is_empty() {
                    match fs::read_dir(".") {
                        Ok(entries) => {
                            let mut file_list = String::new();
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_file() {
                                    if let Ok(name) = entry.file_name().into_string() {
                                        file_list.push_str(&format!("{}\n", name));
                                    }
                                }
                            }
                            let resp = Response::from_string(file_list);
                            let _ = request.respond(resp);
                        }
                        Err(_) => {
                            let resp = Response::from_string("500 Internal Server Error")
                                .with_status_code(500);
                            let _ = request.respond(resp);
                        }
                    }
                } else {
                    let resp = Response::from_string("404 Not Found").with_status_code(404);
                    let _ = request.respond(resp);
                }
            }

            _ => {
                let resp = Response::from_string("405 Method Not Allowed")
                    .with_status_code(405);
                let _ = request.respond(resp);
            }
        }
    }
}
