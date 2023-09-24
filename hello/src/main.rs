mod constants;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use constants::*;
use hello::create_pool;

fn main() {
    let listener = TcpListener::bind(ADDRESS)
        .expect(format!("Expected to bind TcpListener to '{}'.", ADDRESS).as_ref());

    let pool = create_pool(NUM_CPU);

    for stream in listener.incoming() {
        let stream = stream.expect("Expected a TcpStream.");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next()
        .expect("Expected to read request line from buffer.")
        .expect("Expected to read line.");

    let (status_line, filename) = match &request_line[..] {
        GET_ROOT_URI => (STATUS_200_OK, HELLO_HTML),
        GET_SLEEP_URI => {
            sleep(SLEEP_SECS);
            (STATUS_200_OK, SLEEP_HTML)
        }
        _ => (STATUS_404_NOT_FOUND, NOT_FOUND_404_HTML),
    };

    let contents = fs::read_to_string(filename)
        .expect(format!("Expected to read '{}'.", filename).as_ref());
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Expected to write to stream.");
}

fn sleep(secs: u64) {
    // TODO: Implement counting down every second on the "sleep_counter" page that refreshes every second.
    // TODO: At the end, use the regular "sleep" page; this is currently used in handle_connection() anyway.
    thread::sleep(Duration::from_secs(secs));
}

/// This is for debugging purposes only.
fn _print_http_request(buf_reader: BufReader<&mut TcpStream>) {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("Expected to read line."))
        .take_while(|line| !line.is_empty())
        .collect();

    eprintln!("Request: {:#?}", http_request);
}
