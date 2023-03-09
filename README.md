# kb_layout_mgr

Simple utility for managing keyboard layout for Polybar

## Installing

```bash
$ cargo install --git https://github.com/rosengrenen/kb_layout_mgr --profile dist
```

## Polybar config / server

```ini
[module/kb-layout]
type = custom/script

exec = $HOME/.cargo/bin/kb_layout_mgr_server

tail = true
```

## Client

```bash
# Toggle language
$ $HOME/.cargo/bin/kb_layout_mgr_client 0
# Toggle layout
$ $HOME/.cargo/bin/kb_layout_mgr_client 1
```
