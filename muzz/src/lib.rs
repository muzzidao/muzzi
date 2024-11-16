use std::collections::HashMap;

use anyhow::{anyhow, Result};
use kinode_process_lib::{
    await_message, call_init, http, print_to_terminal, println, vfs, Address, LazyLoadBlob, Message,
};

mod messages;
mod types;

use crate::messages::{MuzzError, MuzzRequest, MuzzResponse};
use crate::types::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, Clone, process_macros::SerdeJsonInto],

});

call_init!(init);
fn init(our: Address) {
    println!("muzzidao: welcome to muzz, launching...");

    // bravado print hello messages just to prompt potential future catchup messages and async fetching.
    // would be nice to have this, print a few things and then boom! you're good: )

    // scan local cache? might be a good move... do that before end!
    // TODO::::
    let mut state = State::load(&our);

    let mut http_server = http::server::HttpServer::new(5);

    loop {
        match await_message() {
            Err(send_error) => println!("muzz got SendError: {send_error}"),
            Ok(ref message) => {
                if let Err(e) = handle_message(message, &our, &mut state, &mut http_server) {
                    println!("muzz: encountered handling error: {e}");

                    // FORWARD TO WEBSOCKETS.
                    // EVEN AS A BASIC CONSOLE MESSAGE.
                }
            }
        }
    }
}

pub fn handle_message(
    message: &Message,
    our: &Address,
    state: &mut State,
    http_server: &mut http::server::HttpServer,
) -> Result<()> {
    // match upon just 2 types of messages:
    // 1. requests
    // 2. responses

    if message.is_request() {
        handle_request(message, our, state, http_server)?;
    } else {
        handle_response(message, our, state)?;
    }
    Ok(())
}

pub fn handle_request(
    message: &Message,
    our: &Address,
    state: &mut State,
    http_server: &mut http::server::HttpServer,
) -> Result<()> {
    // matching upon 3 types of requests:
    // 1. requests from other nodes for songs and profile and playlists
    // 2. requests from the frontend (or other ui/raw terminal)
    // 3. request updates from ft_updates

    // foreign message
    if !message.is_local(our) {
    } else {
        // local message
        // if http, handle there, otherwise handle FT_updates :)
        if message.source().process() == "http_server:distro:sys" {
            // forward to http_server

            let server_request =
                serde_json::from_slice::<http::server::HttpServerRequest>(&message.body())?;

            // do methods. GEt all, Post? websockets?
            http_server.handle_request(
                server_request,
                |incoming| handle_http_request(our, state, &incoming),
                |_channel_id, _message_type, _blob| {
                    // not expecting any websocket messages from FE currently
                },
            );
        } else {
            // handle local messages from ft_updates mainly right now.
            // or....?
        }
    }
    Ok(())
}

pub fn handle_response(message: &Message, our: &Address, state: &mut State) -> Result<()> {
    let response = serde_json::from_slice::<MuzzResponse>(&message.body())?;

    match response {
        MuzzResponse::Song(song) => {
            state.hashes_to_songs.insert(song.hash, song);
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_http_request(
    our: &Address,
    state: &mut State,
    req: &http::server::IncomingHttpRequest,
) -> (http::server::HttpResponse, Option<LazyLoadBlob>) {
    match serve_paths(our, state, req) {
        Ok((status_code, _headers, body)) => (
            http::server::HttpResponse::new(status_code).header("Content-Type", "application/json"),
            Some(LazyLoadBlob {
                mime: None,
                bytes: body,
            }),
        ),
        Err(e) => (
            http::server::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            Some(LazyLoadBlob {
                mime: None,
                bytes: serde_json::to_vec(&serde_json::json!({"error": e.to_string()})).unwrap(),
            }),
        ),
    }
}

pub fn serve_paths(
    our: &Address,
    state: &mut State,
    req: &http::server::IncomingHttpRequest,
) -> Result<(http::StatusCode, Option<HashMap<String, String>>, Vec<u8>)> {
    let method = req.method()?;
    let path = req.path()?;

    let bound_path: &str = req.bound_path(Some(&our.process.to_string()));

    // TEMP remove.
    if bound_path == path {
        println!("muzz: WEIRD MISMATCH: {bound_path} != {path}");
    }

    match bound_path {
        "/songs" => {
            println!("muzz: serving songs");
        }
        "/search" => {
            println!("muzz: serving search");
        }
        "/playsongbyhash" => {
            println!("muzz: serving playsongbyhash");
        }
        "/profile" => {
            println!("muzz: serving profile");
        }
        "/playlists" => {
            println!("muzz: serving playlists");
        }
        "/friends" => {
            println!("muzz: serving friends");
        }
        "/settings" => {
            println!("muzz: serving settings");
        }
        "/peers" => {
            println!("muzz: serving peers");
        }
        _ => {
            println!("muzz: unknown path: {bound_path}");
        }
    }

    Ok((http::StatusCode::OK, None, vec![]))
}

pub fn init_frontend(our: &Address, http_server: &mut http::server::HttpServer) {
    let config = http::server::HttpBindingConfig::default();

    for path in [
        "/songs",  // all songs saved (downloaded and not)...?
        "/search", // search local? or also remote... ahhhh this is difficult
        "/profile",
        "/friends",
        "/settings",
        "/peers",
        "/playlists",
        "/playsong",
    ] {
        http_server
            .bind_http_path(path, config.clone())
            .expect("failed to bind http path");
    }

    http_server
        .serve_ui(&our, "ui1", vec!["/"], config.clone())
        .expect("failed to serve static UI");

    // http_server
    //     .serve_ui(&our, "ui2", vec!["/"], config.clone())
    //     .expect("failed to serve static UI");

    // http_server
    //     .serve_ui(&our, "ui3", vec!["/"], config.clone())
    //     .expect("failed to serve static UI");

    http_server
        .bind_ws_path("/", http::server::WsBindingConfig::default())
        .expect("failed to bind ws path");

    // add ourselves to the homepage
    // TODO ADD ICON.

    kinode_process_lib::homepage::add_to_homepage("muzziDAO", None, Some("/"), None);
}
