pub const SOCKET_PATH: &'static str = "/tmp/kb_layout_mgr.socket";

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Undefined,
    ToggleLang,
    ToggleLayout,
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ToggleLang,
            1 => Self::ToggleLayout,
            _ => Self::Undefined,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Lang {
    Swedish,
    English,
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::Swedish => write!(f, "se"),
            Lang::English => write!(f, "us"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Layout {
    Qwerty,
    Colemak,
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layout::Qwerty => write!(f, "qwerty"),
            Layout::Colemak => write!(f, "colemak"),
        }
    }
}

pub fn get_lang() -> Lang {
    let kb_info = get_kb_info();
    if kb_info.contains("us") {
        Lang::English
    } else {
        Lang::Swedish
    }
}

pub fn get_layout() -> Layout {
    let kb_info = get_kb_info();
    if kb_info.contains("colemak") {
        Layout::Colemak
    } else {
        Layout::Qwerty
    }
}

pub fn get_kb_info() -> String {
    run_command_string_output("setxkbmap -query")
}

pub fn run_command(command: &str) -> std::process::Output {
    let parts = command.split_whitespace().collect::<Vec<_>>();
    std::process::Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .expect("Could not execute command")
}

pub fn run_command_string_output(command: &str) -> String {
    let output = run_command(command);
    String::from_utf8(output.stdout).expect("Could not make sense of command output")
}
