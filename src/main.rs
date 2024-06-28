use std::{env, fs};
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let dir_path = env::args().enumerate() // 枚举所有命令行参数
        .find_map(|(i, arg)| {
            if arg == "dir" {
                env::args().nth(i + 1) // 获取 `--dir` 参数后面的值
            } else {
                None
            }
        })
        .map(PathBuf::from) // 将参数值转换为 PathBuf
        .or_else(|| env::current_dir().ok()) // 如果没有 `--dir` 参数，使用当前目录
        .expect("Failed to determine directory");
    let path = Path::new(&dir_path);

    match calculate_directory_size(&path, path) {
        Ok(files) => {
            // 按文件大小排序
            let mut files: Vec<(PathBuf, u64)> = files;
            files.sort_by(|a, b| b.1.cmp(&a.1));

            // 打印排序后的结果
            for (file_path, size) in files {
                println!("{}: {} bytes ≈ {}kb", file_path.display(), size, size / 1024);
            }
        },
        Err(e) => println!("Error calculating directory size: {}", e),
    }
}

fn calculate_directory_size(base_path: &Path, current_path: &Path) -> Result<Vec<(PathBuf, u64)>, io::Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let size = metadata.len();
            // 收集文件路径和大小
            files.push((path.strip_prefix(base_path).unwrap().to_path_buf(), size));
        } else if metadata.is_dir() {
            // 递归收集子目录中的文件
            files.extend(calculate_directory_size(base_path, &path)?);
        }
    }

    Ok(files)
}