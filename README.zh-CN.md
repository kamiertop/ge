# ge

一个用于编写并执行 `git commit -m` 消息的 Git Emoji TUI 工具。

[English](README.md)

## 工作流程

1. 选择或搜索一个 gitmoji。
2. 输入 commit message。
3. 确认最终提交消息。
4. 执行 `git commit -m "<gitmoji> <message>"`。

## 快捷键

- `↑` / `↓` 或 `k` / `j`：移动选择项
- `/`：进入搜索模式
- `Enter`：选择当前 emoji / 确认消息 / 提交
- `Esc`：取消当前步骤
- `q`：在浏览模式退出

## TODO

- [ ] 提交前检查是否存在已暂存的变更。
- [ ] 增加提交前缀格式选项：真实 emoji（`✨`）或 gitmoji code（`:sparkles:`）。
- [ ] 优化常见 Git 状态下的提交失败提示，例如未配置用户信息或没有暂存内容。
- [ ] 增加安装和使用说明。
- [ ] 增加过滤、消息确认、提交消息拼接的测试。
