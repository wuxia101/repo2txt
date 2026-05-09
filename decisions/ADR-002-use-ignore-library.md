# ADR-002: 使用 ignore 库处理 Git ignore 规则

## Status

Accepted

## Context

项目需要正确处理 Git ignore 规则，这包括：
- 读取本地 .gitignore 文件
- 支持全局 Git ignore
- 支持 Git exclude 文件
- 正确实现 ignore 规则匹配逻辑

## Decision

使用 BurntSushi/ignore 库来处理 Git ignore 规则。

## Rationale

ignore 库提供了：
1. 完整的 Git ignore 规则实现
2. 支持本地、全局和 exclude 规则
3. 内置的 Walk 功能，可以在遍历时自动应用 ignore 规则
4. 成熟稳定，被众多 Rust 项目使用
5. 高性能实现

## Consequences

### Positive
- 无需自己实现复杂的 ignore 规则逻辑
- 自动处理所有边缘情况
- 高性能的目录遍历
- 代码更简洁

### Negative
- 增加了外部依赖
- 对库的实现细节了解较少

## Alternatives Considered
- 自己实现 ignore 规则：容易出错，工作量大
- 使用 walkdir + 简单的 glob 匹配：功能不完整
