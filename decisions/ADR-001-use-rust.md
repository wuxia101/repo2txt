# ADR-001: 使用 Rust 作为开发语言

## Status

Accepted

## Context

项目需要一个高性能、可靠的命令行工具，用于处理文件系统操作和遵循 Git ignore 规则。需要考虑的因素包括：
- 性能：处理大型仓库时需要快速
- 可靠性：需要良好的错误处理
- 库生态：需要成熟的库来处理 Git ignore、命令行参数解析等
- 跨平台支持
- 类型安全

## Decision

选择 Rust 作为开发语言。

## Rationale

Rust 提供了以下优势：
1. 高性能，接近 C/C++
2. 强类型系统，编译期捕获大量错误
3. 成熟的 ignore 库，完美处理 Git ignore 规则
4. 优秀的 clap 库用于命令行参数解析
5. anyhow 和 thiserror 提供优雅的错误处理
6. 零成本抽象，没有运行时开销
7. 跨平台支持良好

## Consequences

### Positive
- 高性能和可靠性
- 丰富的生态系统支持文件系统操作
- 编译到原生代码，分发方便
- 内存安全保证

### Negative
- 学习曲线相对较陡
- 编译时间较长

## Alternatives Considered
- Go：有良好的标准库，但错误处理较繁琐
- Python：开发快速，但性能较差
- Node.js：生态丰富，但单线程性能有限
