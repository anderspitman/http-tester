pub struct HttpRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    //body: Vec<u8>,
}

impl HttpRequest {
    pub fn to_string(&self) -> String {

        let mut s = format!("{} {} HTTP/1.1\r\n", self.method, self.path);

        for header_pair in self.headers.clone() {
            let header = header_pair.0;
            let value = header_pair.1;
            s = format!("{}{}: {}\r\n", s, header, value)
        }
        
        s = format!("{}\r\n", s);

        s
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().into()
    }
}

pub struct HttpRequestBuilder {
    request: HttpRequest,
}

impl HttpRequestBuilder {
    pub fn new() -> HttpRequestBuilder {
        HttpRequestBuilder {
            request: HttpRequest {
                method: "GET".to_string(),
                path: "/".to_string(),
                headers: Vec::new(),
                //body: Vec::new(),
            }
        }
    }

    pub fn method(mut self, method: &str) -> HttpRequestBuilder {
        self.request.method = method.to_string();
        self
    }

    pub fn path(mut self, path: &str) -> HttpRequestBuilder {
        self.request.path = path.to_string();
        self
    }

    pub fn header(mut self, header: &str, value: &str) -> HttpRequestBuilder {
        self.request.headers.push((header.to_string(), value.to_string()));
        self
    }

    //fn body(mut self, body: &[u8]) -> HttpRequestBuilder {
    //    self.request.body = body.to_vec();
    //    self
    //}

    pub fn build(self) -> HttpRequest {
        self.request
    }
}
