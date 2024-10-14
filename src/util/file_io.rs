use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

/// 从输入文件复制数据到输出文件
pub fn copy(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut output = OpenOptions::new().write(true).create(true).open(output_path)?;

    let mut buffer = [0u8; 8192]; // 缓冲区大小，类似于 INBUFSIZ
    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // 文件读取完毕
        }
        output.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}

/// 修改文件名，将所有的点替换为下划线，除了最后一个点
pub fn make_simple_name(name: &mut String) {
    if let Some(dot_pos) = name.rfind('.') {
        // 创建一个新的字符串，替换字符后再赋值给原字符串
        let mut new_name = String::with_capacity(name.len());
        for (i, c) in name.chars().enumerate() {
            if c == '.' && i < dot_pos {
                new_name.push('_');
            } else {
                new_name.push(c);
            }
        }
        *name = new_name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_copy() {
        let input_path = PathBuf::from("test_input.txt");
        let output_path = PathBuf::from("test_output.txt");

        fs::write(&input_path, b"Hello, World!").expect("Unable to write test file");
        copy(&input_path, &output_path).expect("Copy operation failed");

        let result = fs::read(&output_path).expect("Unable to read output file");
        assert_eq!(result, b"Hello, World!");

        fs::remove_file(input_path).expect("Unable to remove test input file");
        fs::remove_file(output_path).expect("Unable to remove test output file");
    }

    #[test]
    fn test_make_simple_name() {
        let mut filename = String::from("example.test.file");
        make_simple_name(&mut filename);
        assert_eq!(filename, "example_test.file");
    }
}
