# 项目约定

Rustlings 练习仓库（rustlings 6.5，edition 2024）。练习正文、注释、README 均已翻译为中文，
技术术语（trait、Option、所有权、生命周期、闭包等）保留英文。

## 提交前必做：翻译自动填入的解答

`rustlings` 在练习通过后，会把它**内置的英文解答**（含英文注释）整份写入
`solutions/<章节>/<练习>.rs`。因此每次提交前：

1. 检查 `git status`，看 `solutions/` 下是否有新填入的解答；
2. 先翻译其中的英文注释与说明文字，再和对应的练习文件一起提交；
3. 只改注释与说明文字，代码、标识符、字符串字面量、URL 保持原样。

仍然是占位内容的解答文件（含 `DON'T EDIT THIS SOLUTION FILE!`）不要动：
练习通过时它们会被整份替换，改动它们没有意义。

## 其他注意事项

- `exercises/` 的注释是给学习者看的说明与提示，翻译时不要改动代码、测试与
  被断言的字符串字面量（如 `"Yummy!"`）。
- `rustlings hint` 打印的提示、以及 `solutions/` 里填入的解答，都来自 rustlings
  程序内置数据，改本仓库的文件不会改变它们；`HINTS.zh-CN.md` 是内置提示的中文对照。
- `rustlings reset <练习名>` 会用英文原文覆盖练习文件，若使用过该命令，
  需要按上述规则重新翻译。
- `.rustlings-state.txt` 是本地进度文件，已在 `.gitignore` 中，不要提交。
