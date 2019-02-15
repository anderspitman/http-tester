use std::net::{TcpStream};
use std::io;
use std::io::{Read, Write};
use std::{env, str};
use std::{thread, time};


const BYTES_PER_KBYTE: usize = 1024;
const MAX_CHUNK_BYTES: usize = 16*BYTES_PER_KBYTE;
const MS_PER_SECOND: usize = 1000;
const KBIT_PER_KBYTE: usize = 8;

struct HttpRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    //body: Vec<u8>,
}

impl HttpRequest {
    fn to_string(&self) -> String {

        let mut s = format!("{} {} HTTP/1.1\r\n", self.method, self.path);

        for header_pair in self.headers.clone() {
            let header = header_pair.0;
            let value = header_pair.1;
            s = format!("{}{}: {}\r\n", s, header, value)
        }
        
        s = format!("{}\r\n", s);

        s
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().into()
    }
}

struct HttpRequestBuilder {
    request: HttpRequest,
}

impl HttpRequestBuilder {
    fn new() -> HttpRequestBuilder {
        HttpRequestBuilder {
            request: HttpRequest {
                method: "GET".to_string(),
                path: "/".to_string(),
                headers: Vec::new(),
                //body: Vec::new(),
            }
        }
    }

    fn method(mut self, method: &str) -> HttpRequestBuilder {
        self.request.method = method.to_string();
        self
    }

    fn path(mut self, path: &str) -> HttpRequestBuilder {
        self.request.path = path.to_string();
        self
    }

    fn header(mut self, header: &str, value: &str) -> HttpRequestBuilder {
        self.request.headers.push((header.to_string(), value.to_string()));
        self
    }

    //fn body(mut self, body: &[u8]) -> HttpRequestBuilder {
    //    self.request.body = body.to_vec();
    //    self
    //}

    fn build(self) -> HttpRequest {
        self.request
    }
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid args"));
    }

    let bitrate_kbps = &args[1].parse::<usize>().expect("failed to convert bitrate");
    let delay_ms = (MS_PER_SECOND * KBIT_PER_KBYTE * MAX_CHUNK_BYTES) / bitrate_kbps / BYTES_PER_KBYTE;
    println!("{}", delay_ms);
    let url = &args[2];

    //let host = "localhost";
    //let port = 8081;
    //let host = "lf-proxy.iobio.io";
    //let port = 80;
    let host = "138.68.54.55";
    let port = 9001;

    let req = HttpRequestBuilder::new()
        .method("GET")
        .path(&url)
        .header("Host", host)
        .header("Connection", "close")
        .header("User-Agent", "rusttp")
        .header("Accept", "*/*")
        .build();

    //let stream = TcpStream::connect("lf-proxy.iobio.io:80")?;
    let addr = format!("{}:{}", host, port);
    println!("Sending request to {}:", addr);
    println!("{}", req.to_string());

    let mut stream = TcpStream::connect(&addr)?;
    stream.write(&req.as_bytes())?;

    let chunk_size = 16;
    let mut index = 0;

    loop {

        let mut buf = vec![0; MAX_CHUNK_BYTES];

        //match stream.read(&mut buf[index..(index + chunk_size)]) {
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                //index += n;
            },
            Err(e) => {
                return Err(e);
            },
        }

        io::stdout().write(&buf)?;
        //thread::sleep(time::Duration::from_millis(5));
        thread::sleep(time::Duration::from_millis(delay_ms as u64));
    }

    //println!("{}", str::from_utf8(&buf).expect("error decoding"));

    Ok(())
}
