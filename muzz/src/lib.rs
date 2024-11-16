use std::collections::HashMap;

use kinode_process_lib::{
    await_message, call_init,
    http::server::HttpServer,
    println,
    vfs::{Directory, File, FileMetadata},
    Address,
};
use serde::{Deserialize, Serialize};

wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, Clone, process_macros::SerdeJsonInto],

});

#[derive(Debug, Clone, Serialize, Deserialize)]
struct State {
    /// feel free to improve what part of a song/file is in memory.
    /// vfs::File is fine. vfs::Dir is also fine.
    /// but want some error checking without overwhelming the user.
    hashes_to_songs: HashMap<[u8; 32], Song>,
    /// simplest indexes and lookups I came up with for now.
    /// can add more indices later!
    path_to_songs: HashMap<String, Song>,

    /// playlists
    playlists: Vec<Playlist>,

    /// peers (public)
    /// todo add settings
    /// can extend this, for now can just be a vecdeque/limited size vec
    /// rotate and make feel P2P
    peers: Vec<Peer>,

    /// friends... this is the cool stuff.
    /// contacts:contacts:sys??
    friends: Vec<Friend>,

    /// profile
    /// our personal profile and vinyl bar.
    /// can have hangouts, but mainly we keep the source of truth on our own page, even if shared with others.
    /// ofc, kino.
    /// fuck settings too.
    profile: Profile,

    /// cache
    /// songs are downloaded, but also cached upon every play.
    /// offline first.
    cache: Cache,
}

/// Friend
/// Address, contacts?
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Friend {
    address: Address,
}

/// Profile
/// Address, contacts?
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Profile {
    address: Address,
    posts: Vec<Post>,
}

/// Post on our wall
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    id: u64,
    content: String,
    poster: Address,
}

/// Playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Playlist {
    name: String,
    songs: Vec<Song>,
}

/// Peer
/// Address, latest online check, etc
/// Discovery settings somewhere?
/// we can easily configure ourselves to death hehehe :)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Peer {
    address: Address,
    latest_online: u64,
}

/// Fuck lol we do need some kind of Uuid maybe.
/// if we're storing them like this.
/// Could be a sqlite db also but let's not do that right now.
/// hashes???? just hashes??
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Song {
    hash: [u8; 32],
    path: Option<SongPath>,
    metadata: SongMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SongPath {
    /// local path
    Local(String),
    /// remote path
    Remote(String),
}

/// Metadata... just for a song first?
/// lol hopefully we don't need to version this (OFC WE do. it's a DAO)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SongMetadata {
    title: String,
    artist: String,
    added_by: Address,
    custom_metadata: serde_json::Value,
}

/// we'll do some blake3 content hashing and checking, but nothing extreme yet.
/// a song being duplicated in a playlist, completely fine.
/// Cache with config, SICK.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cache {
    config: CacheConfig,
    // maybe we don't need a dir... ffs.
    //shit should serialize!!!
    // dir: Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum CacheConfig {
    /// Retain a u64 number of bytes in the cache. (downloaded songs that can be played immediately)
    /// Default Cache size in bytes (10GB = 10 * 1024 * 1024 * 1024 bytes)
    Retain(u64), // ~10GB default cache size
}

call_init!(init);
fn init(our: Address) {
    println!("muzzidao: welcome to muzz, launching...");

    // bravado print hello messages just to prompt potential future catchup messages and async fetching.
    // would be nice to have this, print a few things and then boom! you're good: )

    loop {
        match await_message() {
            Err(send_error) => println!("got SendError: {send_error}"),
            Ok(message) => println!("got Message: {message:?}"),
        }
    }
}
