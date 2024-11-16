//! Helper functions for spawning file transfer workers.
//! These functions are used to initiate send and receive operations
//! for file transfers in the App Store system
//!

use kinode_process_lib::*;
use process_macros::SerdeJsonInto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub enum Song {
    Path(String),
    Hash([u8; 32]),
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub struct LocalPlayRequest {
    pub song: Song,
    pub download_from: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub struct RemotePlayRequest {
    pub song: Song,
    pub worker_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub struct ChunkRequest {
    pub song_path: String,
    pub song_hash: [u8; 32],
    pub offset: u64,
    pub length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub struct ProgressUpdate {
    pub song_path: String,
    pub song_hash: [u8; 32],
    pub downloaded: u64,
    pub total: u64,
}

// todo add this back in
// #[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
// pub struct HashMismatch {
//     pub desired: String,
//     pub actual: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub struct DownloadCompleteRequest {
    pub song_path: String,
    pub song_hash: [u8; 32],
    pub err: Option<DownloadError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub enum DownloadError {
    NoPackage,
    NotMirroring,
    FileNotFound,
    WorkerSpawnFailed,
    HttpClientError,
    BlobNotFound,
    VfsError,
    HandlingError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, SerdeJsonInto)]
pub enum DownloadRequests {
    RemotePlay(RemotePlayRequest),
    LocalPlay(LocalPlayRequest),
    Chunk(ChunkRequest),
    Progress(ProgressUpdate),
    DownloadComplete(DownloadCompleteRequest),
    // ... other variants can be added as needed
}

/// Spawns a worker process to send a file transfer.
///
/// This function creates a new worker process, configures it for sending a file,
/// and initiates the transfer to the specified address.
#[allow(dead_code)]
pub fn spawn_send_transfer(
    our: &Address,
    song: Song,
    timeout: u64,
    to_addr: &Address,
) -> anyhow::Result<()> {
    let transfer_id: u64 = rand::random();
    let timer_id = ProcessId::new(Some("timer"), "distro", "sys");
    let Ok(worker_process_id) = spawn(
        Some(&transfer_id.to_string()),
        &format!("{}/pkg/ft_worker.wasm", our.package_id()),
        OnExit::None,
        our_capabilities(),
        vec![timer_id],
        false,
    ) else {
        return Err(anyhow::anyhow!("failed to spawn ft_worker!"));
    };

    let req = Request::new()
        .target((&our.node, worker_process_id))
        .expects_response(timeout + 1)
        .body(
            serde_json::to_vec(&DownloadRequests::RemotePlay(RemotePlayRequest {
                song,
                worker_address: to_addr.to_string(),
            }))
            .unwrap(),
        );
    req.send()?;
    Ok(())
}

/// Spawns a worker process to receive a file transfer.
///
/// This function creates a new worker process, configures it to receive a file
/// from the specified node, and prepares it to handle the incoming transfer.
#[allow(dead_code)]
pub fn spawn_receive_transfer(
    our: &Address,
    song: Song,
    from_node: &str,
    timeout: u64,
) -> anyhow::Result<Address> {
    let transfer_id: u64 = rand::random();
    let timer_id = ProcessId::new(Some("timer"), "distro", "sys");
    let Ok(worker_process_id) = spawn(
        Some(&transfer_id.to_string()),
        &format!("{}/pkg/ft_worker.wasm", our.package_id()),
        OnExit::None,
        our_capabilities(),
        vec![timer_id],
        false,
    ) else {
        return Err(anyhow::anyhow!("failed to spawn ft_worker!"));
    };

    let req = Request::new()
        .target((&our.node, worker_process_id.clone()))
        .expects_response(timeout + 1)
        .body(
            serde_json::to_vec(&DownloadRequests::LocalPlay(LocalPlayRequest {
                song,
                download_from: from_node.to_string(),
            }))
            .unwrap(),
        );

    req.send()?;
    Ok(Address::new(&our.node, worker_process_id))
}
