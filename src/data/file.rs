// src/data/send_file.rs

use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use tokio::io::AsyncWriteExt;

use crate::transport::transport_bytes::{receive_data, send_data};

use super::header::{create_header, parse_header};

const FILE_BUFFER_SIZE: usize = 4096;
const HEADER_SIZE: usize = 272;

pub async fn send_file(addr: &str, file_path: &Path) -> io::Result<()> {
    let mut file = File::open(file_path)?;

    // get the file size
    let file_size = file.metadata()?.len();
    println!("File size: {}", file_size);

    // calculate the number of chunks
    let num_chunks = (file_size as usize + FILE_BUFFER_SIZE - 1) / FILE_BUFFER_SIZE;
    println!("Number of chunks: {}", num_chunks);

    // create the header
    let header = create_header(
        file_size,
        num_chunks as u32,
        FILE_BUFFER_SIZE as u32,
        file_path.to_string_lossy().as_ref(),
    );

    // send the header first
    println!("Sending header: {:?}", &header);
    let mut stream = send_data(addr, header).await?;

    // send the file data in chunks
    let mut buffer = [0u8; FILE_BUFFER_SIZE];
    let mut chunk_count = 0;
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        chunk_count += 1;
        println!("Sending chunk {}/{}", chunk_count, num_chunks);
        stream.write_all(&buffer[..n]).await?;
    }
    Ok(())
}

pub async fn receive_file(addr: &str, dest_path: &Path) -> io::Result<()> {
    let expected_length = HEADER_SIZE + FILE_BUFFER_SIZE; // To begin with header + one chunk
    let data = receive_data(addr, expected_length).await?;

    let (header_bytes, file_data) = data.split_at(HEADER_SIZE);

    // parse the header
    let (file_size, num_chunks, chunk_size, filename) = parse_header(header_bytes);
    println!(
        "Received header: File size: {}, Num chunks: {}, Chunk size: {}, Filename: {}",
        file_size, num_chunks, chunk_size, filename
    );

    let mut file = File::create(dest_path)?;
    let mut pos = 0;
    while pos < file_data.len() {
        let bytes_written = file.write(&file_data[pos..])?;
        pos += bytes_written;
    }

    let remaining_chunks = num_chunks - 1;
    for _ in 0..remaining_chunks {
        let chunk_data = receive_data(addr, chunk_size as usize).await?;
        file.write_all(&chunk_data)?;
    }
    Ok(())
}
