const FILENAME_SIZE: usize = 256;

pub fn create_header(file_size: u64, num_chunks: u32, chunk_size: u32, filename: &str) -> Vec<u8> {
    let mut header = vec![];

    header.extend(&file_size.to_be_bytes());

    header.extend(&num_chunks.to_be_bytes());

    header.extend(&chunk_size.to_be_bytes());

    let filename_bytes = filename.as_bytes();
    let truncated_or_padded: Vec<u8> = if filename_bytes.len() > FILENAME_SIZE {
        filename_bytes[..FILENAME_SIZE].to_vec()
    } else {
        let mut temp = filename_bytes.to_vec();
        temp.resize(FILENAME_SIZE, 0);
        temp
    };
    header.extend(truncated_or_padded);

    header
}

pub fn parse_header(header: &[u8]) -> (u64, u32, u32, String) {
    let (file_size_bytes, rest) = header.split_at(8);
    let file_size =
        u64::from_be_bytes(file_size_bytes.try_into().expect("Incorrect header format"));

    let (num_chunks_bytes, rest) = rest.split_at(4);
    let num_chunks = u32::from_be_bytes(
        num_chunks_bytes
            .try_into()
            .expect("Incorrect header format"),
    );

    let (chunk_size_bytes, rest) = rest.split_at(4);
    let chunk_size = u32::from_be_bytes(
        chunk_size_bytes
            .try_into()
            .expect("Incorrect header format"),
    );

    let filename = String::from_utf8_lossy(&rest[..FILENAME_SIZE])
        .trim_end_matches('\0')
        .to_string();

    (file_size, num_chunks, chunk_size, filename)
}
