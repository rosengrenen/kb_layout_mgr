use std::path::{Path, PathBuf};

use kb_layout::{KbLayoutCommand, SOCKET_PATH};
use status_core::{run_command, run_command_stdout, Server};

struct KbLayoutServer {
    socket_path: PathBuf,
}

impl KbLayoutServer {
    fn new(socket_path: &str) -> Self {
        Self {
            socket_path: PathBuf::from(socket_path),
        }
    }
}

impl Server<KbLayoutCommand, KbLayoutState> for KbLayoutServer {
    fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    fn format_output(&self) -> String {
        let (lang, layout) = self.state();
        format!("[{} {}]", lang, layout)
    }

    fn state(&self) -> KbLayoutState {
        (get_lang(), get_layout())
    }

    fn handle_command(&self, command: KbLayoutCommand) {
        let (lang, layout) = self.state();
        match command {
            KbLayoutCommand::ToggleLang => toggle_lang(lang, layout),
            KbLayoutCommand::ToggleLayout => toggle_layout(lang, layout),
            KbLayoutCommand::Undefined => (),
        };
    }
}

fn main() {
    let server = KbLayoutServer::new(SOCKET_PATH);
    server.run();
}

fn toggle_lang(lang: Lang, layout: Layout) {
    let new_lang = match lang {
        Lang::Swedish => Lang::English,
        Lang::English => Lang::Swedish,
    };

    set_lang_and_layout(new_lang, layout);
}

fn toggle_layout(lang: Lang, layout: Layout) {
    let new_layout = match layout {
        Layout::Qwerty => Layout::Colemak,
        Layout::Colemak => Layout::Qwerty,
    };

    set_lang_and_layout(lang, new_layout);
}

fn set_lang_and_layout(lang: Lang, layout: Layout) {
    match (lang, layout) {
        (Lang::Swedish, Layout::Qwerty) => run_command("setxkbmap -layout se"),
        (Lang::Swedish, Layout::Colemak) => run_command("setxkbmap -layout colemak_dh_se"),
        (Lang::English, Layout::Qwerty) => run_command("setxkbmap -layout us"),
        (Lang::English, Layout::Colemak) => run_command("setxkbmap -layout us -variant colemak_dh"),
    };
}

type KbLayoutState = (Lang, Layout);

#[derive(Clone, Copy, Debug)]
enum Lang {
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
enum Layout {
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

fn get_lang() -> Lang {
    let kb_info = get_kb_info();
    if kb_info.contains("us") {
        Lang::English
    } else {
        Lang::Swedish
    }
}

fn get_layout() -> Layout {
    let kb_info = get_kb_info();
    if kb_info.contains("colemak") {
        Layout::Colemak
    } else {
        Layout::Qwerty
    }
}

fn get_kb_info() -> String {
    run_command_stdout("setxkbmap -query")
}
