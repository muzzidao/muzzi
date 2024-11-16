use kinode_process_lib::{Address, Message};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MuzzRequest {
    GetSongByHash([u8; 32]),
    GetSongByPath(String), // should be improved... /artist/song
    GetProfile,
    GetPlaylists,
    GetSongs,
    GetCachedSongs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MuzzResponse {
    Err(MuzzError),
    Song(Song),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MuzzError {
    SongNotFound,
    ProfileNotFound,
    PlaylistNotFound,
    SongNotCached,
    // ...
}

impl std::error::Error for MuzzError {}

impl fmt::Display for MuzzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MuzzError::SongNotFound => write!(f, "Song not found"),
            MuzzError::ProfileNotFound => write!(f, "Profile not found"),
            MuzzError::PlaylistNotFound => write!(f, "Playlist not found"),
            MuzzError::SongNotCached => write!(f, "Song not cached"),
        }
    }
}
