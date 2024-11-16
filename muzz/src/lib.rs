use std::collections::HashMap;

use anyhow::{anyhow, Result};
use kinode_process_lib::{
    await_message, call_init, http, print_to_terminal, println, vfs, Address, LazyLoadBlob,
    Message, Request, Response,
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

    init_frontend(&our, &mut http_server);

    loop {
        match await_message() {
            Err(send_error) => println!("muzz got SendError: {send_error}"),
            Ok(ref message) => {
                if let Err(e) = handle_message(message, &our, &mut state, &mut http_server) {
                    println!("muzz: encountered handling error: {e}");

                    // FORWARD TO WEBSOCKETS.
                    // EVEN AS A BASIC CONSOLE MESSAGE.
                    http_server.ws_push_all_channels(
                        "/",
                        http::server::WsMessageType::Text,
                        LazyLoadBlob {
                            mime: Some("application/json".to_string()),
                            bytes: serde_json::to_vec(&serde_json::json!({
                                "kind": "error",
                                "error": e.to_string(),
                            }))
                            .unwrap(),
                        },
                    );
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
        handle_response(message, our, state, http_server)?;
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
        let request = serde_json::from_slice::<MuzzRequest>(&message.body())?;
        match request {
            MuzzRequest::GetSongByHash(hash) => {
                if let Some(song) = state.hashes_to_songs.get(&hash) {
                    // TODO! SPIN UP WORKER; SEND WITH THAT!
                    if state.settings.public_song_sharing {
                        Response::new()
                            .body(&MuzzResponse::Song(song.clone()))
                            .send()?;
                    } else {
                        Response::new()
                            .body(&MuzzResponse::Err(MuzzError::SongNotFound))
                            .send()?;
                    }
                } else {
                    Response::new()
                        .body(&MuzzResponse::Err(MuzzError::SongNotFound))
                        .send()?;
                }
            }
            MuzzRequest::GetSongByPath(path) => {
                if let Some(song) = state.path_to_songs.get(&path) {
                    // TODO! SPIN UP WORKER; SEND WITH THAT!
                    Response::new()
                        .body(&MuzzResponse::Song(song.clone()))
                        .send()?;
                } else {
                    Response::new()
                        .body(&MuzzResponse::Err(MuzzError::SongNotFound))
                        .send()?;
                }
            }
            MuzzRequest::GetProfile => {
                Response::new()
                    .body(&MuzzResponse::Profile(state.profile.clone()))
                    .send()?;
            }
            MuzzRequest::GetPlaylists => {
                Response::new()
                    .body(&MuzzResponse::Playlists(state.playlists.clone()))
                    .send()?;
            }
            MuzzRequest::GetSongs => {
                let songs: Vec<Song> = state.hashes_to_songs.values().cloned().collect();
                Response::new().body(&MuzzResponse::Songs(songs)).send()?;
            }
            MuzzRequest::GetFriends => {
                Response::new()
                    .body(&MuzzResponse::Friends(state.friends.clone()))
                    .send()?;
            }
            MuzzRequest::GetPeers => {
                Response::new()
                    .body(&MuzzResponse::Peers(state.peers.clone()))
                    .send()?;
            }
            MuzzRequest::GetSettings => {
                Response::new()
                    .body(&MuzzResponse::Settings(state.settings.clone()))
                    .send()?;
            }
            MuzzRequest::AddSong(song) => {
                state.hashes_to_songs.insert(song.hash, song.clone());
                if let Some(path) = &song.path {
                    match path {
                        SongPath::Local(p) => {
                            state.path_to_songs.insert(p.clone(), song);
                        }
                        SongPath::Remote(p) => {
                            state.path_to_songs.insert(p.clone(), song);
                        }
                    }
                }
                Response::new().body(&MuzzResponse::Ok).send()?;
            }
            MuzzRequest::AddPlaylist(playlist) => {
                state.playlists.push(playlist);
                Response::new().body(&MuzzResponse::Ok).send()?;
            }
            MuzzRequest::AddFriend(friend) => {
                state.friends.push(friend);
                Response::new().body(&MuzzResponse::Ok).send()?;
            }

            MuzzRequest::AddPost(post) => {
                if state.settings.public_wall {
                    state.profile.posts.push(post);
                    Response::new().body(&MuzzResponse::Ok).send()?;
                } else {
                    Response::new()
                        .body(&MuzzResponse::Err(MuzzError::WallNotPublic))
                        .send()?;
                }
            }
            MuzzRequest::GetCachedSongs => {
                // actual files now baby!
                // Response::new()
                //     .body(&MuzzResponse::Songs(state.cached_songs.clone()))
                //     .send()?;
            }
            MuzzRequest::Search(_query) => {
                // TODO!
                // Response::new()
                //     .body(&MuzzResponse::Songs(state.cached_songs.clone()))
                //     .send()?;
            }
            MuzzRequest::GetProfiles => {
                Response::new()
                    .body(&MuzzResponse::Profiles(
                        state
                            .profiles
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect::<Vec<_>>(),
                    ))
                    .send()?;
            }
        }
    } else {
        // local message
        // if http, handle there, otherwise handle FT_updates :)
        if message.source().process() == "http_server:distro:sys" {
            // forward to http_server
            let server_request =
                serde_json::from_slice::<http::server::HttpServerRequest>(&message.body())?;

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

pub fn handle_response(
    message: &Message,
    our: &Address,
    state: &mut State,
    http_server: &mut http::server::HttpServer,
) -> Result<()> {
    let response = serde_json::from_slice::<MuzzResponse>(&message.body())?;

    match response {
        MuzzResponse::Song(song) => {
            state.hashes_to_songs.insert(song.hash, song.clone());
            if let Some(path) = &song.path {
                match path {
                    SongPath::Local(p) => {
                        state.path_to_songs.insert(p.clone(), song);
                    }
                    SongPath::Remote(p) => {
                        state.path_to_songs.insert(p.clone(), song);
                    }
                }
            }
        }
        MuzzResponse::Profile(profile) => {
            state.profile = profile;
        }
        MuzzResponse::Playlists(playlists) => {
            state.playlists = playlists;
        }
        MuzzResponse::Songs(songs) => {
            for song in songs {
                state.hashes_to_songs.insert(song.hash, song.clone());
                if let Some(path) = &song.path {
                    match path {
                        SongPath::Local(p) => {
                            state.path_to_songs.insert(p.clone(), song);
                        }
                        SongPath::Remote(p) => {
                            state.path_to_songs.insert(p.clone(), song);
                        }
                    }
                }
            }
        }
        MuzzResponse::Friends(friends) => {
            state.friends = friends;
        }
        MuzzResponse::Peers(peers) => {
            state.peers = peers;
        }
        MuzzResponse::Settings(settings) => {
            state.settings = settings;
        }
        MuzzResponse::Playlist(playlist) => {
            // Find and update existing playlist or add new one
            if let Some(existing) = state.playlists.iter_mut().find(|p| p.name == playlist.name) {
                *existing = playlist;
            } else {
                state.playlists.push(playlist);
            }
        }
        MuzzResponse::Profiles(profiles) => {
            // TODO.
            // for (address, profile) in profiles {
            //     state.profiles.insert(address, profile);
            // }
        }
        MuzzResponse::Ok => {
            // Success response, no state update needed
        }
        MuzzResponse::Err(err) => {
            // FORWARD TO WEBSOCKETS.
            // EVEN AS A BASIC CONSOLE MESSAGE.
            http_server.ws_push_all_channels(
                "/",
                http::server::WsMessageType::Text,
                LazyLoadBlob {
                    mime: Some("application/json".to_string()),
                    bytes: serde_json::to_vec(&serde_json::json!({
                        "kind": "error",
                        "error": err.to_string(),
                    }))
                    .unwrap(),
                },
            );
            println!("muzz: received error response: {}", err);
        }
    }

    // Save state after any modifications
    // state.save();
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

    match (method.as_str(), bound_path) {
        // GET endpoints
        ("GET", "/songs") => {
            let songs: Vec<Song> = state.hashes_to_songs.values().cloned().collect();
            Ok((http::StatusCode::OK, None, serde_json::to_vec(&songs)?))
        }
        ("GET", "/search") => {
            if let Some(query) = req.query_params().get("q") {
                // TODO: Implement actual search
                Ok((http::StatusCode::OK, None, vec![]))
            } else {
                Ok((
                    http::StatusCode::BAD_REQUEST,
                    None,
                    serde_json::to_vec(&MuzzError::InvalidQuery)?,
                ))
            }
        }
        ("GET", "/playsongbyhash") => {
            if let Some(hash_str) = req.query_params().get("hash") {
                let hash: [u8; 32] = hex::decode(hash_str)
                    .map_err(|_| MuzzError::InvalidHash)?
                    .try_into()
                    .map_err(|_| MuzzError::InvalidHash)?;
                if let Some(song) = state.hashes_to_songs.get(&hash) {
                    Ok((http::StatusCode::OK, None, serde_json::to_vec(&song)?))
                } else {
                    Ok((
                        http::StatusCode::NOT_FOUND,
                        None,
                        serde_json::to_vec(&MuzzError::SongNotFound)?,
                    ))
                }
            } else {
                Ok((
                    http::StatusCode::BAD_REQUEST,
                    None,
                    serde_json::to_vec(&MuzzError::SongNotFound)?,
                ))
            }
        }
        ("GET", "/profile") => Ok((
            http::StatusCode::OK,
            None,
            serde_json::to_vec(&state.profile)?,
        )),
        ("GET", "/playlists") => Ok((
            http::StatusCode::OK,
            None,
            serde_json::to_vec(&state.playlists)?,
        )),
        ("GET", "/friends") => Ok((
            http::StatusCode::OK,
            None,
            serde_json::to_vec(&state.friends)?,
        )),
        _ => {
            println!("muzz: unknown path: {bound_path}");
            Ok((http::StatusCode::NOT_FOUND, None, vec![]))
        }
    }
}

pub fn init_frontend(our: &Address, http_server: &mut http::server::HttpServer) {
    let config = http::server::HttpBindingConfig::default();

    for path in [
        "/songs",  // all songs saved (downloaded and not)...?
        "/search", // search local? or also remote... ahhhh this is difficult
        "/profile",
        "/profiles",
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
