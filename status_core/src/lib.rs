use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::{Path, PathBuf},
};

pub trait Command {
    fn from_u8(value: u8) -> Self;
    fn to_u8(&self) -> u8;
}

pub struct Client {
    socket_path: PathBuf,
}

impl Client {
    pub fn new(socket_path: &str) -> Self {
        Self {
            socket_path: PathBuf::from(socket_path),
        }
    }

    pub fn send_command<C>(&self, command: C)
    where
        C: Command,
    {
        let mut stream = match UnixStream::connect(&self.socket_path) {
            Err(_) => panic!("Server is not running"),
            Ok(stream) => stream,
        };

        match stream.write(&[command.to_u8()]) {
            Err(_) => panic!("Could not send message"),
            Ok(_) => {}
        }
    }

    pub fn send_command_from_env<C>(&self)
    where
        C: Command,
    {
        let args = std::env::args().collect::<Vec<_>>();
        let command = C::from_u8(args[1].as_bytes()[0] - b'0');
        self.send_command(command);
    }
}

pub trait Server<C, S>: Sized
where
    C: Command,
{
    fn socket_path(&self) -> &Path;

    fn format_output(&self) -> String;

    fn state(&self) -> S;

    fn handle_command(&self, command: C);

    fn run(self) {
        let socket_path = self.socket_path();

        // Delete old socket if necessary
        if socket_path.exists() {
            std::fs::remove_file(&socket_path).expect("Could not remove existing socket file");
        }

        // Bind to socket
        let stream = match UnixListener::bind(&socket_path) {
            Err(_) => panic!("Failed to bind socket"),
            Ok(stream) => stream,
        };

        println!("{}", self.format_output());

        // Iterate over clients, blocks if no client available
        let mut buf = [0; 1];
        for client in stream.incoming() {
            let mut client = match client {
                Ok(client) => client,
                _ => continue,
            };

            match client.read(&mut buf) {
                Ok(len) if len > 1 => self.handle_command(Command::from_u8(buf[0])),
                _ => continue,
            }

            println!("{}", self.format_output());
        }
    }
}

/// Runs a command formatted as a single string, by splitting it up into pieces
pub fn run_command(command: &str) -> std::process::Output {
    let parts = command.split_whitespace().collect::<Vec<_>>();
    std::process::Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .expect("Could not execute command")
}

pub fn run_command_stdout(command: &str) -> String {
    let parts = command.split_whitespace().collect::<Vec<_>>();
    let output = std::process::Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .expect("Could not execute command");
    String::from_utf8(output.stdout).unwrap()
}
