#[derive(Debug, Default)]
pub struct Record<'doc_info> {
    pub tag_id: u16,
    pub size: usize,
    pub payload: &'doc_info [u8],
    pub children: Vec<Record<'doc_info>>,
}

#[derive(Debug, Error)]
pub enum RecordError {
    #[error("Decompression error: {0}")]
    Decompression(#[from] std::io::Error),
    #[error("Invalid doc info")]
    InvalidDocInfo,
    #[error("Invalid record level: {0}")]
    InvalidRecordLevel(u16),
}

pub fn inflate<'doc_info>(
    mut bytes: &'doc_info [u8],
) -> Result<Vec<Record<'doc_info>>, RecordError> {
    let mut root = Record::default();
    while !bytes.is_empty() {
        let record;
        let level;
        (record, level, bytes) = consume(bytes);

        let mut parent = &mut root;
        for _ in 0..level {
            match parent.children.last_mut() {
                Some(record) => parent = record,
                None => return Err(RecordError::InvalidRecordLevel(level)),
            };
        }
        parent.children.push(record);
    }

    Ok(root.children)
}

fn consume(bytes: &[u8]) -> (Record, u16, &[u8]) {
    const OVER_SIZED: usize = 0xFFF;

    let header = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

    let tag_id = (header & 0x3FF) as u16;
    let level = ((header >> 10) & 0x3FF) as u16;
    let size = ((header >> 20) & 0xFFF) as usize;

    let (size, payload, bytes) = match size {
        OVER_SIZED => {
            let size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
            let (payload, bytes) = bytes[8..].split_at(size);

            (size, payload, bytes)
        }
        size => {
            let (payload, bytes) = bytes[4..].split_at(size);

            (size, payload, bytes)
        }
    };
    let record = Record {
        tag_id,
        size,
        payload,
        children: vec![],
    };

    (record, level, bytes)
}
