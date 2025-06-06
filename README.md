# 使用 Tauri & 🦀 + Vue + TypeScript 的 Todo List

一个**自用的任务清单**  
灵感来自 Minecraft 科技整合包中常见的写字板 *上线、在机器上跳来跳去、下线*  
使用了 Minecraft 本体和机械动力 Mod 的 UI 贴图和音效资源

## 数据存储

由于出现了「日志放在管理员权限文件夹 *比如 ProgramFiles* 下会直接让整个应用在非管理员权限下无法打开」的 bug  
所以只能 ~~被迫~~ 往 `%APPDATA%` 里**拉屎**了  
在这里列出来也是想着「用户如果觉得删不干净的话可以手动清理」这种想法

具体文件路径如图：

```text
%AppData%
└─ Local
   └─ tauri-todo-list
      ├─ app.log
      └─ tasks.toml
```

其中 `app.log` 会保存每一次打开应用后的日志，下一次打开时就**清空**并重新记录  
一般情况里面只有十几行日志

## QA

### Q: 为什么要限定任务栏位 & 任务组数量

A: 根据我的个人习惯，如果按照其他 TODO 那样可以无限制增加新任务，那么我会**堆积大量未完成的任务**不了了之，所以干脆限定可用的任务数量。如果你需要添加新的任务，最好**把当前任务做完**。

### Q: 为什么要用 Rust

A: `Cargo, run!`

### Q: 为什么有 Python 混进来了

A: `update_version.py` 字面意思，拿来更新版本号的脚本 ~~肯定拿脚本语言写脚本对吧~~，使用 `python update_version.py` 运行。
