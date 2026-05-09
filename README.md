# repo2txt

将Git仓库结构和内容转换为单个文本文件的工具，包含目录树输出和文件内容的Markdown代码块。非常适合与大语言模型(LLM)交流代码库。

## 功能特性

- 📁 **智能目录遍历**：递归遍历Git仓库目录结构
- 🚫 **Gitignore支持**：自动遵循.gitignore规则过滤文件
- 🔍 **二进制文件检测**：自动识别并跳过二进制文件
- 📝 **结构化输出**：清晰的文件路径和内容组织
- 🎯 **自定义配置**：支持自定义输出路径和详细程度
- ⚡ **高性能**：基于Rust语言开发，处理大仓库也能保持快速

## 快速开始

### 安装

```bash
cargo install repo2txt
```

### 基本使用

```bash
# 转换当前目录到repo.txt
repo2txt

# 转换指定目录到自定义输出文件
repo2txt --source /path/to/repo --output myrepo.txt

# 包含二进制文件（不推荐）
repo2txt --include-binary

# 显示详细输出
repo2txt --verbose
```

## 命令行参数

```
Usage: repo2txt [OPTIONS]

Options:
  -s, --source <SOURCE>    源目录路径（Git仓库根目录） [default: .]
  -o, --output <OUTPUT>    输出文件路径 [default: repo.txt]
  -b, --include-binary     是否包含二进制文件
  -v, --verbose            是否显示详细输出
  -h, --help               Print help
  -V, --version            Print version
```

## 输出格式

输出文件包含两部分：
1. **目录树**：显示完整的仓库结构
2. **文件内容**：每个文件的路径和内容，使用Markdown代码块包裹

示例输出：

```
📁 my-repo/
├── 📄 README.md
├── 📄 Cargo.toml
└── 📁 src/
    └── 📄 main.rs

---

## 📄 README.md
```markdown
# my-repo
这是一个Rust项目
```

---

## 📄 src/main.rs
```rust
fn main() {
    println!("Hello, World!");
}
```
```

## 技术架构

- **语言**：Rust
- **核心依赖**：
  - `clap`：CLI参数解析
  - `walkdir`：目录遍历
  - `ignore`：Gitignore规则处理
  - `anyhow`：错误处理

## 开发流程

本项目遵循Source-Bound SpecFlow开发规范：
1. 需求定义在`docs/`目录
2. 架构决策记录在`decisions/`目录
3. 代码与需求严格绑定
4. 使用`specflow` CLI进行验证

## 许可证

MIT
