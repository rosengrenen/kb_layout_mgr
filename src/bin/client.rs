use std::{env, io::Write, os::unix::net::UnixStream, path::Path};

use kb_layout_mgr::SOCKET_PATH;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let command = args[1].as_bytes()[0] - b'0';

    let socket = Path::new(SOCKET_PATH);
    let mut stream = match UnixStream::connect(&socket) {
        Err(_) => panic!("Server is not running"),
        Ok(stream) => stream,
    };

    match stream.write(&[command]) {
        Err(_) => panic!("Couldn't send message"),
        Ok(_) => {}
    }
}
