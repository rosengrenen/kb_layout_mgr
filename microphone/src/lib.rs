use status_core::Command;

pub const SOCKET_PATH: &'static str = "/tmp/microphone.socket";

#[derive(Clone, Copy, Debug)]
pub enum MicrophoneCommand {
    Undefined,
    ToggleMute,
}

impl Command for MicrophoneCommand {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::ToggleMute,
            _ => Self::Undefined,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            MicrophoneCommand::ToggleMute => 0,
            MicrophoneCommand::Undefined => 1,
        }
    }
}
