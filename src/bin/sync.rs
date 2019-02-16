use std::net::{TcpStream};
use std::io;
use std::io::{Read, Write};
use std::{env};
use std::{thread, time};
use url::{Url};
use http_tester::{HttpRequestBuilder};


const BYTES_PER_KBYTE: usize = 1024;
const MAX_CHUNK_BYTES: usize = 16*BYTES_PER_KBYTE;
const MS_PER_SECOND: usize = 1000;
const KBIT_PER_KBYTE: usize = 8;


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    eprintln!("{:?}", args);
    if args.len() != 3 {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid args"));
    }

    let bitrate_kbps = &args[1].parse::<usize>().expect("failed to convert bitrate");
    let delay_ms = (MS_PER_SECOND * KBIT_PER_KBYTE * MAX_CHUNK_BYTES) / bitrate_kbps / BYTES_PER_KBYTE;
    eprintln!("{}", delay_ms);
    let url = Url::parse(&args[2]).expect("failed parsing url");

    let host = url.host_str().expect("failed parsing host");

    let req = HttpRequestBuilder::new()
        .method("GET")
        .path(&url.path())
        .header("Host", host)
        .header("Connection", "close")
        .header("User-Agent", "rusttp")
        .header("Accept", "*/*")
        .build();

    let addr = match url.port() {
        Some(port) => format!("{}:{}", host, port),
        None => host.to_string(),
    };

    eprintln!("{}", req.to_string());

    eprintln!("{:?}", addr);

    let mut stream = TcpStream::connect(&addr)?;
    stream.write(&req.as_bytes())?;

    loop {

        let mut buf = vec![0; MAX_CHUNK_BYTES];

        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
            },
            Err(e) => {
                return Err(e);
            },
        }

        io::stdout().write(&buf)?;
        //thread::sleep(time::Duration::from_millis(5));
        thread::sleep(time::Duration::from_millis(delay_ms as u64));
    }

    Ok(())
}
