use status_core::Command;

pub const SOCKET_PATH: &'static str = "/tmp/kb_layout.socket";

#[derive(Clone, Copy, Debug)]
pub enum KbLayoutCommand {
    Undefined,
    ToggleLang,
    ToggleLayout,
}

impl Command for KbLayoutCommand {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::ToggleLang,
            1 => Self::ToggleLayout,
            _ => Self::Undefined,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            KbLayoutCommand::ToggleLang => 0,
            KbLayoutCommand::ToggleLayout => 1,
            KbLayoutCommand::Undefined => 2,
        }
    }
}
