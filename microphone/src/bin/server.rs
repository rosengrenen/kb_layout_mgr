use std::path::{Path, PathBuf};

use microphone::{MicrophoneCommand, SOCKET_PATH};
use status_core::{run_command, run_command_stdout, Server};

struct MicrophoneServer {
    socket_path: PathBuf,
}

impl MicrophoneServer {
    fn new(socket_path: &str) -> Self {
        Self {
            socket_path: PathBuf::from(socket_path),
        }
    }
}

impl Server<MicrophoneCommand, MicrophoneState> for MicrophoneServer {
    fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    fn format_output(&self) -> String {
        let microphone_status = self.state();
        format!("[{}]", microphone_status)
    }

    fn state(&self) -> MicrophoneState {
        get_microphone_status()
    }

    fn handle_command(&self, command: MicrophoneCommand) {
        match command {
            MicrophoneCommand::ToggleMute => toggle_mute(),
            MicrophoneCommand::Undefined => (),
        };
    }
}

fn main() {
    let server = MicrophoneServer::new(SOCKET_PATH);
    server.run();
}

fn toggle_mute() {
    run_command("wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle");
}

type MicrophoneState = MicrophoneStatus;

#[derive(Clone, Copy, Debug)]
enum MicrophoneStatus {
    On,
    Muted,
}

impl std::fmt::Display for MicrophoneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MicrophoneStatus::On => write!(f, "on"),
            MicrophoneStatus::Muted => write!(f, "muted"),
        }
    }
}

fn get_microphone_status() -> MicrophoneStatus {
    let kb_info = run_command_stdout("wpctl get-volume @DEFAULT_AUDIO_SOURCE@");
    if kb_info.contains("MUTED") {
        MicrophoneStatus::Muted
    } else {
        MicrophoneStatus::On
    }
}
