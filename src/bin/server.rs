use std::{io::Read, os::unix::net::UnixListener, path::Path};

use kb_layout_mgr::{get_lang, get_layout, run_command, Command, Lang, Layout, SOCKET_PATH};

fn main() {
    let socket = Path::new(SOCKET_PATH);

    // Delete old socket if necessary
    if socket.exists() {
        std::fs::remove_file(&socket).expect("Could not remove existing socket file");
    }

    // Bind to socket
    let stream = match UnixListener::bind(&socket) {
        Err(_) => panic!("Failed to bind socket"),
        Ok(stream) => stream,
    };

    let mut lang = get_lang();
    let mut layout = get_layout();

    println!("[{} {}]", lang, layout);

    // Iterate over clients, blocks if no client available
    let mut buf = [0; 1];
    for client in stream.incoming() {
        let mut client = match client {
            Ok(client) => client,
            _ => continue,
        };

        lang = get_lang();
        layout = get_layout();

        match client.read(&mut buf) {
            Ok(len) if len == 1 => match Command::from(buf[0]) {
                Command::ToggleLang => lang = toggle_lang(lang, layout),
                Command::ToggleLayout => layout = toggle_layout(lang, layout),
                Command::Undefined => (),
            },
            _ => continue,
        }

        println!("[{} {}]", lang, layout);
    }
}

fn toggle_lang(lang: Lang, layout: Layout) -> Lang {
    let new_lang = match lang {
        Lang::Swedish => Lang::English,
        Lang::English => Lang::Swedish,
    };

    set_lang_and_layout(new_lang, layout);

    new_lang
}

fn toggle_layout(lang: Lang, layout: Layout) -> Layout {
    let new_layout = match layout {
        Layout::Qwerty => Layout::Colemak,
        Layout::Colemak => Layout::Qwerty,
    };

    set_lang_and_layout(lang, new_layout);

    new_layout
}

fn set_lang_and_layout(lang: Lang, layout: Layout) {
    match (lang, layout) {
        (Lang::Swedish, Layout::Qwerty) => run_command("setxkbmap -layout se"),
        (Lang::Swedish, Layout::Colemak) => run_command("setxkbmap -layout colemak_dh_se"),
        (Lang::English, Layout::Qwerty) => run_command("setxkbmap -layout us"),
        (Lang::English, Layout::Colemak) => run_command("setxkbmap -layout us -variant colemak_dh"),
    };
}
