//!
//! @anarizzza
//! NOTE: (code from kinode core app_store) with a slight modification for muzzi, first chunk is sent as 10kb, next ones 50kb

use kinode_process_lib::*;
use kinode_process_lib::{
    print_to_terminal, println, timer,
    vfs::{File, SeekFrom},
};
// use sha2::{Digest, Sha256};
use std::io::Read;
use std::str::FromStr;

pub mod ft_worker_lib;

use crate::ft_worker_lib::{
    ChunkRequest, DownloadCompleteRequest, DownloadError, DownloadRequests, LocalPlayRequest,
    ProgressUpdate, RemotePlayRequest, Song,
};

wit_bindgen::generate!({
    path: "target/wit",
    generate_unused_types: true,
    world: "process-v0",
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

const CHUNK_SIZE: u64 = 51200; // 50KB
                               // const FIRST_CHUNK_SIZE: u64 = 10240; // 10KB

call_init!(init);
fn init(our: Address) {
    let Ok(Message::Request {
        source: parent_process,
        body,
        ..
    }) = await_message()
    else {
        panic!("ft_worker: got bad init message");
    };

    if parent_process.node() != our.node() {
        panic!("ft_worker: got bad init message source");
    }
}
//     // killswitch timer, 2 minutes. sender or receiver gets killed/cleaned up.
//     // TODO: killswitch update bubbles up to downloads process?
//     timer::set_timer(120000, None);

//     let start = std::time::Instant::now();

//     match body
//         .try_into()
//         .expect("ft_worker: got unparseable init message")
//     {
//         DownloadRequests::LocalPlay(local_request) => {
//             let LocalPlayRequest {
//                 song,
//                 download_from,
//             } = local_request;
//             match handle_receiver(&parent_process, &song) {
//                 Ok(_) => print_to_terminal(
//                     1,
//                     &format!(
//                         "ft_worker: receive downloaded song in {}ms",
//                         start.elapsed().as_millis()
//                     ),
//                 ),
//                 Err(e) => print_to_terminal(1, &format!("ft_worker: receive error: {}", e)),
//             }
//         }
//         DownloadRequests::RemotePlay(remote_request) => {
//             let RemotePlayRequest {
//                 song,
//                 worker_address,
//             } = remote_request;

//             match handle_sender(&worker_address, &song) {
//                 Ok(_) => print_to_terminal(
//                     1,
//                     &format!(
//                         "ft_worker: sent song to {} in {}ms",
//                         worker_address,
//                         start.elapsed().as_millis()
//                     ),
//                 ),
//                 Err(e) => print_to_terminal(1, &format!("ft_worker: send error: {}", e)),
//             }
//         }
//         _ => println!("ft_worker: got unexpected message"),
//     }
// }

// fn handle_sender(worker: &str, song: &Song) -> anyhow::Result<()> {
//     let target_worker = Address::from_str(worker)?;

//     let filename = match song {
//         Song::Path(path) => path.clone(),
//         Song::Hash(hash) => format!("/muzik:app/songs/{:x}.mp3", hash),
//     };

//     let mut file = vfs::open_file(&filename, false, None)?;
//     let size = file.metadata()?.len;
//     let num_chunks = (size as f64 / CHUNK_SIZE as f64).ceil() as u64;

//     file.seek(SeekFrom::Start(0))?;

//     for i in 0..num_chunks {
//         send_chunk(&mut file, i, size, &target_worker, &filename, song)?;
//     }

//     Ok(())
// }

// fn handle_receiver(parent_process: &Address, song: &Song) -> anyhow::Result<()> {
//     let timer_address = Address::from_str("our@timer:distro:sys")?;

//     let mut file: Option<File> = None;
//     let mut size: Option<u64> = None;
//     let mut hasher = Sha256::new();

//     let song_path = match song {
//         Song::Path(path) => path.clone(),
//         Song::Hash(hash) => format!("/muzik:app/songs/{:x}.mp3", hash),
//     };

//     loop {
//         let message = await_message()?;
//         if *message.source() == timer_address {
//             return Ok(());
//         }
//         if !message.is_request() {
//             return Err(anyhow::anyhow!("ft_worker: got bad message"));
//         }

//         match message.body().try_into()? {
//             DownloadRequests::Chunk(chunk) => {
//                 let bytes = if let Some(blob) = get_blob() {
//                     blob.bytes
//                 } else {
//                     return Err(anyhow::anyhow!("ft_worker: got no blob in chunk request"));
//                 };

//                 if file.is_none() {
//                     file = Some(vfs::open_file(&song_path, true, None)?);
//                 }

//                 handle_chunk(
//                     file.as_mut().unwrap(),
//                     &chunk,
//                     parent_process,
//                     &mut size,
//                     &mut hasher,
//                     &bytes,
//                 )?;
//                 if let Some(s) = size {
//                     if chunk.offset + chunk.length >= s {
//                         let recieved_hash = format!("{:x}", hasher.finalize());

//                         if recieved_hash != song.hash() {
//                             print_to_terminal(
//                                 1,
//                                 &format!(
//                                     "ft_worker: {} hash mismatch: desired: {} != actual: {}",
//                                     song.path(),
//                                     song.hash(),
//                                     recieved_hash
//                                 ),
//                             );
//                             let req = DownloadCompleteRequest {
//                                 song_path: song_path.clone(),
//                                 song_hash: song.hash().to_string(),
//                                 err: Some(DownloadError::HashMismatch(HashMismatch {
//                                     desired: song.hash().to_string(),
//                                     actual: recieved_hash,
//                                 })),
//                             };
//                             Request::new()
//                                 .body(DownloadRequests::DownloadComplete(req))
//                                 .target(parent_process.clone())
//                                 .send()?;
//                         }

//                         let manifest_filename = format!("{}{}.json", song_path, song.hash());

//                         let contents = file.as_mut().unwrap().read()?;
//                         extract_and_write_manifest(&contents, &manifest_filename)?;

//                         Request::new()
//                             .body(DownloadRequests::DownloadComplete(
//                                 DownloadCompleteRequest {
//                                     song_path: song_path.clone(),
//                                     song_hash: song.hash().to_string(),
//                                     err: None,
//                                 },
//                             ))
//                             .target(parent_process.clone())
//                             .send()?;
//                         return Ok(());
//                     }
//                 }
//             }
//             DownloadRequests::Size(update) => {
//                 size = Some(update.size);
//             }
//             _ => println!("ft_worker: got unexpected message"),
//         }
//     }
// }

// fn send_chunk(
//     file: &mut File,
//     chunk_index: u64,
//     total_size: u64,
//     target: &Address,
//     song_path: &str,
//     song: &Song,
// ) -> anyhow::Result<()> {
//     let offset = chunk_index * CHUNK_SIZE;
//     let length = CHUNK_SIZE.min(total_size - offset);

//     let mut buffer = vec![0; length as usize];
//     file.seek(SeekFrom::Start(offset))?;
//     file.read_at(&mut buffer)?;

//     Request::new()
//         .body(DownloadRequests::Chunk(ChunkRequest {
//             song_path: song_path.to_string(),
//             song_hash: match song {
//                 Song::Hash(hash) => *hash,
//                 Song::Path(_) => [0u8; 32], // or compute hash if needed
//             },
//             offset,
//             length,
//         }))
//         .target(target.clone())
//         .blob_bytes(buffer)
//         .send()?;
//     Ok(())
// }

// fn handle_chunk(
//     file: &mut File,
//     chunk: &ChunkRequest,
//     parent: &Address,
//     size: &mut Option<u64>,
//     hasher: &mut Sha256,
//     bytes: &[u8],
// ) -> anyhow::Result<()> {
//     file.write_all(bytes)?;
//     hasher.update(bytes);

//     if let Some(total_size) = size {
//         // let progress = ((chunk.offset + chunk.length) as f64 / *total_size as f64 * 100.0) as u64;

//         Request::new()
//             .body(DownloadRequests::Progress(ProgressUpdate {
//                 song_path: chunk.song_path.clone(),
//                 song_hash: chunk.song_hash.clone(),
//                 downloaded: chunk.offset + chunk.length,
//                 total: *total_size,
//             }))
//             .target(parent.clone())
//             .send()?;
//     }

//     Ok(())
// }

// fn extract_and_write_manifest(file_contents: &[u8], manifest_path: &str) -> anyhow::Result<()> {
//     let reader = std::io::Cursor::new(file_contents);
//     let mut archive = zip::ZipArchive::new(reader)?;

//     for i in 0..archive.len() {
//         let mut file = archive.by_index(i)?;
//         if file.name() == "manifest.json" {
//             let mut contents = String::new();
//             file.read_to_string(&mut contents)?;

//             let manifest_file = vfs::open_file(&manifest_path, true, None)?;
//             manifest_file.write(contents.as_bytes())?;

//             print_to_terminal(1, "Extracted and wrote manifest.json");
//             break;
//         }
//     }

//     Ok(())
// }
