use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref BASE_DLPATH: PathBuf = {
        let path = PathBuf::from("server/src/img");
        fs::create_dir_all(&path).expect("Failed to create img directory");
        path
    };
}

pub fn send_file(stream: &mut TcpStream, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let f: File = File::open(BASE_DLPATH.join(filename))?;
    // let mut reader = BufReader::new(f);
    // io::copy(&mut reader, stream)?;
    let f: Vec<u8> = std::fs::read(BASE_DLPATH.join(filename))?;
    stream.write_all(f.as_ref())?;

    println!(
        "[INFO] Sending {} to client stream",
        BASE_DLPATH.join(filename).to_str().unwrap()
    );

    Ok(())
}

pub fn resolve_filename(filename: &mut str) -> String {
    if filename.contains(".png") {
        return String::from(filename);
    }

    filename.to_string().push_str(".png");
    String::from(filename)
}
