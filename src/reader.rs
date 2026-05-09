//! 文件读取模块
//! 
//! Business: true
//! Requirement: REQ-003
//! Purpose: 读取文件内容，处理文本编码
//! Layer: domain
//! Boundary: Reader
//! Flow: 文件转换流程

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

/// 读取文本文件内容
/// 
/// Business: true
/// Requirement: REQ-003
/// Purpose: 读取文本文件内容
/// Layer: domain
/// Boundary: Reader
/// Flow: 文件转换流程
/// 
/// # Arguments
/// 
/// * `file_path` - 文件路径
/// 
/// # Returns
/// 
/// 返回文件内容的String
pub fn read_text_file(file_path: &Path) -> Result<String> {
    fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))
}

/// 判断文件是否可能是二进制文件
/// 
/// Business: true
/// Requirement: REQ-003
/// Purpose: 判断文件是否可能是二进制文件
/// Layer: domain
/// Boundary: Reader
/// Flow: 文件转换流程
/// 
/// # Arguments
/// 
/// * `file_path` - 文件路径
/// 
/// # Returns
/// 
/// 返回是否可能是二进制文件
pub fn is_binary_file(file_path: &Path) -> bool {
    if let Ok(bytes) = fs::read(file_path) {
        // 简单判断：前1000个字节中如果有NUL字节，认为是二进制文件
        let sample = &bytes[..std::cmp::min(1000, bytes.len())];
        sample.contains(&0)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_read_text_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let content = "Hello, world!";
        fs::write(&file_path, content).unwrap();
        
        let read_content = read_text_file(&file_path).unwrap();
        assert_eq!(read_content, content);
    }
}
