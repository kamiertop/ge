# ge

A Git Emoji TUI for composing and running `git commit -m` messages.

[简体中文](README.zh-CN.md)

## Workflow

1. Select or search for a gitmoji.
2. Enter a commit message.
3. Review the final commit message.
4. Run `git commit -m "<gitmoji> <message>"`.

## Shortcuts

- `↑` / `↓` or `k` / `j`: move selection
- `/`: enter search mode
- `Enter`: select current emoji / confirm message / commit
- `Esc`: cancel current step
- `q`: quit from browse mode

## TODO

- [ ] Add a guard before commit to check whether staged changes exist.
- [ ] Add an option to choose commit prefix format: emoji icon (`✨`) or gitmoji code (`:sparkles:`).
- [ ] Improve commit failure messages for common Git states, such as missing user config or nothing staged.
- [ ] Add install and usage instructions.
- [ ] Add tests for filtering, message confirmation, and commit message composition.
