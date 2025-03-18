#[derive(Debug)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Message::Text(text) => format!("TEXT|{}", text).into_bytes(),
            Message::Binary(bytes) => {
                let header = format!("BINARY|{}\n", bytes.len());
                let mut content_bytes = header.into_bytes();
                content_bytes.extend_from_slice(bytes);
                content_bytes
            }
        }
    }

    pub fn decode(raw: &[u8]) -> Option<Message> {
        let raw_str = String::from_utf8_lossy(raw);
        if raw_str.starts_with("TEXT|") {
            Some(Message::Text(
                raw_str.trim_start_matches("TEXT|").trim().to_string(),
            ))
        } else if raw_str.starts_with("BINARY|") {
            let parts: Vec<&str> = raw_str.splitn(2, "\n").collect();
            if parts.len() == 2 {
                let size: usize = parts[0].trim_end_matches("BINARY|").parse().ok()?;
                Some(Message::Binary(parts[1].as_bytes()[..size].to_vec()))
            } else {
                None
            }
        } else {
            None
        }
    }
}
