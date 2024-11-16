use kinode_process_lib::{get_state, set_state};
use std::collections::HashMap;

use kinode_process_lib::{println, Address};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// feel free to improve what part of a song/file is in memory.
    /// vfs::File is fine. vfs::Dir is also fine.
    /// but want some error checking without overwhelming the user.
    pub hashes_to_songs: HashMap<[u8; 32], Song>,
    /// simplest indexes and lookups I came up with for now.
    /// can add more indices later!
    pub path_to_songs: HashMap<String, Song>,

    /// playlists
    pub playlists: Vec<Playlist>,

    /// peers (public)
    /// todo add settings
    /// can extend this, for now can just be a vecdeque/limited size vec
    /// rotate and make feel P2P
    pub peers: Vec<Peer>,

    /// friends... this is the cool stuff.
    /// contacts:contacts:sys??
    pub friends: Vec<Friend>,

    /// profile
    /// our personal profile and vinyl bar.
    /// can have hangouts, but mainly we keep the source of truth on our own page, even if shared with others.
    /// ofc, kino.
    /// fuck settings too.
    pub profile: Profile,

    /// general settings for now
    pub settings: Settings,
}

/// settings

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub public_song_sharing: bool, // wether you fulfill requests from remote nodes for hashes... don't be a leech!
    pub public_wall: bool,         // whether ppl can post on your wall
    pub cache_config: CacheConfig, // songs are downloaded, but also cached upon every play.
}

/// Friend
/// Address, contacts?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
    pub address: Address,
}

/// Profile
/// Address, contacts?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub address: Address,
    pub posts: Vec<Post>,
}

/// Post on our wall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub content: String,
    pub poster: Address,
}

/// Playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>,
}

/// Peer
/// Address, latest online check, etc
/// Discovery settings somewhere?
/// we can easily configure ourselves to death hehehe :)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub address: Address,
    pub latest_online: u64,
}

/// Fuck lol we do need some kind of Uuid maybe.
/// if we're storing them like this.
/// Could be a sqlite db also but let's not do that right now.
/// hashes???? just hashes??
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub hash: [u8; 32],
    pub path: Option<SongPath>,
    pub metadata: SongMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SongPath {
    /// local path
    Local(String),
    /// remote path
    Remote(String),
}

/// Metadata... just for a song first?
/// lol hopefully we don't need to version this (OFC WE do. it's a DAO)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongMetadata {
    pub title: String,
    pub artist: String,
    pub added_by: Address,
    pub custom_metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheConfig {
    /// Retain a u64 number of bytes in the cache. (downloaded songs that can be played immediately)
    /// Default Cache size in bytes (10GB = 10 * 1024 * 1024 * 1024 bytes)
    Retain(u64), // ~10GB default cache size
}

// // //
// // // IMPLS
// // //

impl State {
    pub fn new(our: &Address) -> Self {
        Self {
            hashes_to_songs: HashMap::new(),
            path_to_songs: HashMap::new(),
            playlists: vec![],
            peers: vec![],
            friends: vec![],
            profile: Profile {
                address: our.clone(),
                posts: vec![],
            },
            settings: Settings {
                public_song_sharing: true,
                public_wall: true,
                cache_config: CacheConfig::Retain(10737418240), // 10GB
            },
        }
    }

    pub fn load(our: &Address) -> Self {
        if let Some(blob) = get_state() {
            if let Ok(state) = serde_json::from_slice(&blob) {
                state
            } else {
                println!("muzz: failed to load state, resetting");
                Self::new(our)
            }
        } else {
            Self::new(our)
        }
    }

    pub fn save(&self) {
        // should never fail I think? yolo.
        set_state(&serde_json::to_vec(self).unwrap());
    }
}
