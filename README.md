# Nbt-to-JSON

将基岩版**nbt**文件转换为**json**格式（不包含标签类型，长度），此项目仅用于个人学习，本人不会进行维护！

### 安装

你可以下载最新的发新版(CLI)或克隆仓库运行```cargo build```编译可执行文件。

得益于**WebAssembly**，你也可以在线使用**Nbt-to-JSON**。

- [Release链接](https://github.com/simple-mq/nbt-to-json/releases)

### 使用

#### CLI: 

命令行界面下键入```nbt-to-json --help```获取帮助信息。

**example:**

```shell
./nbt-to-json ./example.nbt --pretty

//output:
//{
//    "Root": {
//        "String Tag": "example test!"
//    }
//}
```

#### 在线使用：

- [Netlify](https://nbt-to-json.netlify.app/)

### 注意事项

- 输出JSON时会将第一个**Compoud**标签的名称记为**Root**
