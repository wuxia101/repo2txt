//! repo2txt - Git仓库到单个TXT文件转换工具
//! 
//! Business: true
//! Requirement: REQ-001
//! Requirement: REQ-002
//! Requirement: REQ-003
//! Requirement: REQ-004
//! Requirement: REQ-005
//! Purpose: 主入口文件，处理CLI参数并协调整个转换流程
//! Layer: application
//! Boundary: CLI
//! Flow: 文件转换流程
//! Decision: ADR-001-use-rust
//! Decision: ADR-002-use-ignore-library
//! Decision: ADR-003-output-format

use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

mod traversal;
mod reader;
mod merger;

/// Git仓库到TXT文件转换工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 源目录路径（Git仓库根目录）
    #[arg(short, long, default_value = ".")]
    source: PathBuf,
    
    /// 输出文件路径
    #[arg(short, long, default_value = "repo.txt")]
    output: PathBuf,
    
    /// 是否包含二进制文件
    #[arg(short, long)]
    include_binary: bool,
    
    /// 是否显示详细输出
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.verbose {
        println!("Processing repository: {:?}", args.source);
        println!("Output file: {:?}", args.output);
    }
    
    convert_repo_to_txt(&args)?;
    
    Ok(())
}

/// 将Git仓库转换为单个TXT文件
/// 
/// Business: true
/// Requirement: REQ-001
/// Requirement: REQ-002
/// Requirement: REQ-003
/// Requirement: REQ-004
/// Requirement: REQ-005
/// Purpose: 协调各个模块完成整个转换过程
/// Layer: application
/// Boundary: CLI
/// Flow: 文件转换流程
fn convert_repo_to_txt(args: &Args) -> Result<()> {
    // 1. 遍历目录（自动应用.gitignore规则）
    let all_files = traversal::traverse_directory(&args.source)?;
    
    // 2. 处理文件
    let mut valid_entries = Vec::new();
    
    for file_path in all_files {
        // 计算相对路径
        let relative_path = file_path.strip_prefix(&args.source)
            .unwrap_or(&file_path)
            .to_path_buf();
        
        if args.verbose {
            println!("Processing: {:?}", relative_path);
        }
        
        // 检查是否为二进制文件
        if !args.include_binary && reader::is_binary_file(&file_path) {
            if args.verbose {
                println!("Skipping binary file: {:?}", relative_path);
            }
            continue;
        }
        
        // 读取文件内容
        match reader::read_text_file(&file_path) {
            Ok(content) => {
                valid_entries.push(merger::FileEntry::new(relative_path, content));
            }
            Err(e) => {
                if args.verbose {
                    eprintln!("Warning: Could not read file {:?}: {}", file_path, e);
                }
            }
        }
    }
    
    // 3. 合并文件
    merger::merge_files(&valid_entries, &args.source, &args.output)?;
    
    println!("Successfully processed {} files", valid_entries.len());
    
    Ok(())
}
