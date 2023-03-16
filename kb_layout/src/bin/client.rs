use kb_layout::{KbLayoutCommand, SOCKET_PATH};
use status_core::Client;

fn main() {
    let client = Client::new(SOCKET_PATH);
    client.send_command_from_env::<KbLayoutCommand>();
}
