---
title: Vscode 配置
categories: Config
---

## setting.json

```json
{
  "explorer.compactFolders": false, // 文件是否紧凑显示

  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode" //ts默认格式化
  },

  "[javascript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode" // js默认格式化
  },

  "[jsonc]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },

  "editor.formatOnSave": true, // 保存之后是否格式化

  "editor.tabSize": 2, // tab缩进

  "merge-conflict.autoNavigateNextConflict.enabled": true, // 是否在解决合并冲突后自动转到下一个合并冲突

  "files.autoSave": "afterDelay", // 延迟自动保存

  "files.autoSaveDelay": 5000, // 等待5秒自动保存

  "editor.bracketPairColorization.enabled": true, // 彩虹括号对

  "editor.wordWrapColumn": 150, // 单词换行

  "editor.lineHeight": 25, // 编辑器行高

  "files.trimTrailingWhitespace": true // 保存文件时候删除行尾的空格
}
```

## 常用插件

- Code Spell Checker

- Easy Snippet

- IntelliJ IDEA Keybindings

- Stylelint

- Prettier - Code formatter

- EsLint

- Code Translate

- Error Lens

- EditorConfig for VS Code

---

### 单测插件

- Jest Runner

---

### vue 插件

- Vetur

- Vue Language Feature (Volar)

- Vite

### 其他插件

- CodeSnap

- Butter Comments
  > TODO 等注释显示
- DotENV

- Quokkas
  > 在线运行

### 主题

- Material Icon Theme
  > 文件夹主题

## vscode setting.json

```json
{
  "explorer.compactFolders": false,
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[javascript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[jsonc]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "editor.formatOnSave": true,
  "editor.tabSize": 2,
  "merge-conflict.autoNavigateNextConflict.enabled": true,
  "files.autoSave": "afterDelay",
  "files.autoSaveDelay": 5000,
  "editor.bracketPairColorization.enabled": true,
  "editor.wordWrapColumn": 150,
  "files.trimTrailingWhitespace": true,
  "workbench.iconTheme": "material-icon-theme"
}
```
