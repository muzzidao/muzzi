use kinode_process_lib::{Address, Message};
use process_macros::SerdeJsonInto;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub enum MuzzRequest {
    GetSongByHash([u8; 32]),
    GetSongByPath(String), // should be improved... /artist/song
    GetProfile,
    GetProfiles,
    GetPlaylists,
    GetSongs,
    GetCachedSongs,
    GetFriends,
    GetPeers,
    GetSettings,
    AddSong(Song),
    AddPlaylist(Playlist),
    AddFriend(Friend),
    Search(String),
    AddPost(Post),
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub enum MuzzResponse {
    Err(MuzzError),
    Ok,
    Song(Song),
    Playlist(Playlist),
    Profile(Profile),
    Profiles(Vec<(Address, Profile)>),
    Songs(Vec<Song>),
    Playlists(Vec<Playlist>),
    Friends(Vec<Friend>),
    Peers(Vec<Peer>),
    Settings(Settings),
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub enum MuzzError {
    SongNotFound,
    ProfileNotFound,
    PlaylistNotFound,
    SongNotCached,
    WallNotPublic,
    InvalidQuery,
    InvalidHash,
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
            MuzzError::WallNotPublic => write!(f, "Wall not public"),
            MuzzError::InvalidQuery => write!(f, "Invalid query"),
            MuzzError::InvalidHash => write!(f, "Invalid hash"),
        }
    }
}
