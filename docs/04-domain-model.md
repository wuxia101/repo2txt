# 领域模型文档

## 核心领域概念

### 1. GitRepository
表示一个git仓库，包含以下属性：
- path: 仓库根目录路径
- ignore_rules: .gitignore规则集合
- files: 仓库中的文件列表

### 2. FileEntry
表示一个文件条目，包含以下属性：
- path: 文件相对路径
- absolute_path: 文件绝对路径
- content: 文件内容
- is_binary: 是否为二进制文件

### 3. IgnoreRule
表示一个.gitignore规则，包含以下属性：
- pattern: 规则模式
- negation: 是否为否定规则（以!开头）
- directory: 是否为目录规则（以/结尾）

### 4. ConversionConfig
转换配置，包含以下属性：
- source_dir: 源目录路径
- output_file: 输出文件路径
- include_binary: 是否包含二进制文件
- verbose: 是否显示详细输出

## 领域关系
- GitRepository 包含多个 FileEntry
- GitRepository 有一个 IgnoreRules 集合
- ConversionConfig 用于配置转换过程

## 核心行为
1. GitRepository::scan() - 扫描仓库中的文件
2. IgnoreRules::is_ignored() - 判断文件是否被忽略
3. FileEntry::read_content() - 读取文件内容
4. ContentMerger::merge() - 合并文件内容