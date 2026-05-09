# ADR-003: 使用自定义 XML-like 格式作为输出

## Status

Accepted

## Context

需要选择一种格式将多个文件的内容合并到单个文件中，要求：
- 清晰标识文件边界
- 包含文件路径信息
- 便于解析和阅读
- 不与常见文件内容冲突

## Decision

使用自定义的 XML-like 标签格式，包含 `<repo-to-text>`、`<directory_structure>` 和 `<content>` 标签。

## Rationale

这种格式提供了：
1. 清晰的文件分隔和标识
2. 包含完整的文件路径信息
3. 人眼可读性良好
4. 简单的标签结构，易于解析
5. 避免与常见代码内容冲突（使用尖括号但不是完整的 XML）

格式结构：
```
<repo-to-text>
Directory: .

Directory Structure:
<directory_structure>
file1.txt
dir/file2.txt
</directory_structure>

<content full_path="file1.txt">
file content here
</content>

</repo-to-text>
```

## Consequences

### Positive
- 清晰易读
- 易于解析
- 包含完整的目录结构信息
- 与用户需求完全匹配

### Negative
- 需要自定义解析器
- 不是标准格式

## Alternatives Considered
- 纯文本，使用文件路径作为分隔符：不够结构化
- JSON：对人眼不够友好
- XML：过于复杂，可能与某些文件内容冲突
- Tar/Zip 格式：二进制，不可直接阅读
