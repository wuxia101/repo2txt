//! 内容合并模块
//! 
//! Business: true
//! Requirement: REQ-004
//! Purpose: 将所有文件内容合并为单个txt文件，添加文件路径标识
//! Layer: domain
//! Boundary: Merger
//! Flow: 文件转换流程
//! Decision: ADR-003-output-format

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;
use anyhow::{Context, Result};

/// 文件内容条目
/// 
/// Business: true
/// Requirement: REQ-004
/// Purpose: 表示一个文件的内容条目
/// Layer: domain
/// Boundary: Merger
/// Domain: FileContent
pub struct FileEntry {
    pub relative_path: PathBuf,
    pub content: String,
}

impl FileEntry {
    /// 创建新的文件内容条目
    /// 
    /// Support: true
    /// Purpose: 创建新的文件内容条目
    pub fn new(relative_path: PathBuf, content: String) -> Self {
        Self {
            relative_path,
            content,
        }
    }
}

/// 合并文件内容到输出文件
/// 
/// Business: true
/// Requirement: REQ-004
/// Purpose: 将所有文件内容合并为单个txt文件，添加文件路径标识
/// Layer: domain
/// Boundary: Merger
/// Flow: 文件转换流程
/// 
/// # Arguments
/// 
/// * `entries` - 文件条目列表
/// * `source_dir` - 源目录路径（用于显示相对路径）
/// * `output_path` - 输出文件路径
/// 
/// # Returns
/// 
/// 返回成功或错误
pub fn merge_files(
    entries: &[FileEntry],
    source_dir: &Path,
    output_path: &Path,
) -> Result<()> {
    let mut file = File::create(output_path)
        .with_context(|| format!("Failed to create output file: {:?}", output_path))?;
    
    // 写入头部信息
    writeln!(file, "<repo-to-text>")?;
    writeln!(file, "Directory: {}", source_dir.to_string_lossy())?;
    writeln!(file)?;
    
    // 写入目录结构
    writeln!(file, "Directory Structure:")?;
    writeln!(file, "<directory_structure>")?;
    // 简单的目录结构表示
    for entry in entries {
        writeln!(file, "{}", entry.relative_path.to_string_lossy())?;
    }
    writeln!(file, "</directory_structure>")?;
    writeln!(file)?;
    
    // 写入文件内容
    for entry in entries {
        writeln!(file, "<content full_path=\"{}\">", entry.relative_path.to_string_lossy())?;
        writeln!(file, "{}", entry.content)?;
        writeln!(file, "</content>")?;
        writeln!(file)?;
    }
    
    writeln!(file, "</repo-to-text>")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_merge_files() {
        let source_dir = tempdir().unwrap();
        let output_file = tempfile::NamedTempFile::new().unwrap();
        
        let entries = vec![
            FileEntry::new(
                PathBuf::from("test.txt"),
                "Hello, world!".to_string(),
            ),
        ];
        
        merge_files(&entries, source_dir.path(), output_file.path()).unwrap();
        
        // 简单验证文件被创建
        assert!(output_file.path().exists());
    }
}
