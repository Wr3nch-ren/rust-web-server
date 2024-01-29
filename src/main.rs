use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use web_server::ThreadPool;

fn main() {
    // Bind the listener to the port
    // "bind" returns a new Result<T, E> type instance (Ok, Err)
    // P.S. non-admin users can only bind ports above 1023
    // unwrap() will return the value inside the Ok() or panic! if Err
    // tldr; unwrap() will stop the program if there is an error
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Create thread pool for handling connections
    let pool = ThreadPool::new(4);

    // Iterate over incoming connections into a TcpStream
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // Handle each connection using each thread in the pool
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // Create a buffer reader from the stream
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Check if the request is a GET request
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "test.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Read the file
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Create the response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    // Write the response to the stream
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
