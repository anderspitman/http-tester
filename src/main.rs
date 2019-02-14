use std::net::{TcpStream};
use std::io;
use std::io::{Read, Write};
use std::{env, str};
use std::{thread, time};


const MAX_RESPONSE_BYTES: usize = 10000;

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
    if args.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid args"));
    }

    let url = &args[1];

    //let host = "localhost";
    //let port = 8081;
    let host = "lf-proxy.iobio.io";
    let port = 80;

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

    let mut buf = vec![0; MAX_RESPONSE_BYTES];
    let chunk_size = 16;
    let mut index = 0;

    loop {
        match stream.read(&mut buf[index..(index + chunk_size)]) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                println!("\n");
                index += n;
            },
            Err(e) => {
                return Err(e);
            },
        }

        io::stdout().write(&buf)?;
        thread::sleep(time::Duration::from_millis(100));
    }

    //println!("{}", str::from_utf8(&buf).expect("error decoding"));

    Ok(())
}
