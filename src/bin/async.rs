use tokio::prelude::*;
use tokio::io;
use tokio::net::TcpStream;
use http_tester::{HttpRequestBuilder};

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:9001".parse().expect("fail parsing address");
    let client = TcpStream::connect(&addr).and_then(|stream| {

        let request = HttpRequestBuilder::new()
            .method("GET")
            //.path(&url.path())
            .path("/")
            .header("Host", "localhost")
            .header("Connection", "close")
            .header("User-Agent", "rusttp")
            .header("Accept", "*/*")
            .build();

        eprintln!("{}", request.to_string());

        io::write_all(stream, request.as_bytes()).then(|result| {
          println!("wrote to stream; success={:?}", result.is_ok());
          Ok(())
        })
    })
    .map_err(|err| {
        println!("connection error = {:?}", err);
    });

    tokio::run(client);

    Ok(())
}
