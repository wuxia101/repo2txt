//! 文件遍历模块
//! 
//! Business: true
//! Requirement: REQ-001
//! Requirement: REQ-002
//! Purpose: 递归遍历目录结构，收集所有文件路径，同时应用.gitignore规则
//! Layer: domain
//! Boundary: Traversal
//! Flow: 文件转换流程
//! Decision: ADR-002-use-ignore-library

use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use anyhow::Result;

/// 遍历目录，收集所有文件路径，同时应用.gitignore规则
/// 
/// Business: true
/// Requirement: REQ-001
/// Requirement: REQ-002
/// Purpose: 递归遍历目录结构，收集所有文件路径，同时应用.gitignore规则
/// Layer: domain
/// Boundary: Traversal
/// Flow: 文件转换流程
/// 
/// # Arguments
/// 
/// * `source_dir` - 源目录路径
/// 
/// # Returns
/// 
/// 返回包含所有文件路径的Vec
pub fn traverse_directory(source_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    let walk = WalkBuilder::new(source_dir)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();
    
    for result in walk {
        let entry = result?;
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            files.push(entry.path().to_path_buf());
        }
    }
    
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_traverse_directory() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();
        
        let files = traverse_directory(dir.path()).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }
}
