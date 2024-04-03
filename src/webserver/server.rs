use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");

        // 2. handle stream
        //simple_handle_connection(stream);

        handle_connection(stream);

        // 1. when the stream goes out of scope, then the connection will be drop
        // which the client will receive 'connection reset' message.
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

pub fn simple_handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        // take_while(): After `false` is returned, `take_while()`'s job is over,
        // and the rest of the elements are ignored.
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);
    // stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("src/webserver/hello.html").unwrap();
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
