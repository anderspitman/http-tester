use std::net::{TcpStream};
use std::io;
use std::io::{Read, Write};
use std::str;


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

    //let host = "localhost";
    //let port = 8081;
    let host = "lf-proxy.iobio.io";
    let port = 80;

    let req = HttpRequestBuilder::new()
        .method("GET")
        .path("/")
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
    stream.read(&mut buf)?;

    println!("{}", str::from_utf8(&buf).expect("error decoding"));

    Ok(())
}
