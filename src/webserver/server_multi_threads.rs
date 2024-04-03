use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self},
    time::Duration,
};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| {
            //handle_connection(stream);
        })
    }
}

pub struct ThreadPool {
    count: i32,
}
impl ThreadPool {
    pub fn new(count: i32) -> ThreadPool {
        ThreadPool { count }
    }

    pub fn execute<F>(&self, function: F)
    where
        F: Fn(),
    {
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next();
    if let Some(req_line_result) = request_line {
        match req_line_result {
            Ok(req_line) => {
                let (status_line, filename) = match &req_line[..] {
                    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/webserver/hello.html"),
                    "GET /sleep HTTP/1.1" => {
                        thread::sleep(Duration::from_secs(5));
                        ("HTTP/1.1 200 OK", "src/webserver/hello.html")
                    }
                    _ => ("HTTP/1.1 404 NOT FOUND", "src/webserver/404.html"),
                };

                response_page(status_line, filename, stream);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    } else {
        println!("Connection closed");
    }
}

fn response_page(status_line: &str, name: &str, mut stream: TcpStream) {
    let contents = fs::read_to_string(name).unwrap();
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server() {
        start_server();
    }
}
