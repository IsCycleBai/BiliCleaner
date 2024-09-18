use fancy_regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

// 定义布尔常量来控制是否输出日志
const DEBUG_FLAG: bool = false;

/// 从指定目录中读取规则文件，并返回正则表达式列表
pub fn load_regex_rules(directory: &str) -> Result<Vec<Regex>, Box<dyn Error>> {
    let mut regex_rules = Vec::new();

    // 遍历目录中的所有文件
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        // 仅处理 .re 文件
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("re") {
            if let Some(mut regex_list) = process_file(&path)? {
                regex_rules.append(&mut regex_list);
            }
        }
    }

    Ok(regex_rules)
}

/// 处理每个文件，检查是否包含 #BiliCleaner/regexRules，并返回编译后的正则表达式列表
fn process_file(path: &PathBuf) -> Result<Option<Vec<Regex>>, Box<dyn Error>> {
    // 读取文件内容
    let content = fs::read_to_string(path)?;
    if DEBUG_FLAG {
        println!("Processing file: {:?}", path.display());
        println!("File content:\n{}", content);
    }

    // 检查文件是否以 #BiliCleaner/regexRules 开头
    if content.starts_with("#BiliCleaner/regexRules") {
        let mut regex_list = Vec::new();
        let mut current_pattern = String::new();
        let mut continuation = false;

        for line in content.lines().skip(1) { // 跳过第一行注释
            let trimmed = line.trim();

            // 忽略空行和以 # 开头的行
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // 如果上一行以 \ 结尾，则移除结尾的反斜杠并拼接当前行
            if continuation {
                current_pattern.pop(); // 移除结尾的反斜杠
                current_pattern.push_str(trimmed);
                if DEBUG_FLAG {
                    println!("Appending to current pattern (continuation): {}", trimmed);
                }
            } else {
                current_pattern.push_str(trimmed);
                if DEBUG_FLAG {
                    println!("Appending to current pattern: {}", trimmed);
                }
            }

            // 检查当前行是否以反斜杠结尾
            continuation = trimmed.ends_with('\\');

            // 如果当前行不是续行且当前模式不为空，则编译并存储正则表达式
            if !continuation && !current_pattern.is_empty() {
                let final_regex_pattern = format!("(?s){}", current_pattern);
                if DEBUG_FLAG {
                    println!("Final regex pattern: {}", final_regex_pattern);
                }

                match Regex::new(&final_regex_pattern) {
                    Ok(regex) => {
                        regex_list.push(regex);
                        current_pattern.clear(); // 清空当前模式以开始处理下一个
                    },
                    Err(e) => {
                        eprintln!("Error compiling regex pattern: {}", e);
                    }
                }
            }
        }

        // 处理最后一个模式（如果有的话）
        if !current_pattern.is_empty() {
            let final_regex_pattern = format!("(?s){}", current_pattern);
            if DEBUG_FLAG {
                println!("Final regex pattern: {}", final_regex_pattern);
            }

            match Regex::new(&final_regex_pattern) {
                Ok(regex) => regex_list.push(regex),
                Err(e) => eprintln!("Error compiling regex pattern: {}", e),
            }
        }

        if regex_list.is_empty() {
            Ok(None)
        } else {
            Ok(Some(regex_list))
        }
    } else {
        Ok(None)
    }
}

/// 匹配给定文本是否符合任何加载的正则表达式
pub fn check_text_against_rules(rules: &[Regex], text: &str) -> bool {
    for rule in rules {
        match rule.is_match(text) {
            Ok(is_match) => {
                if is_match {
                    return true;
                }
            }
            Err(e) => eprintln!("Error matching text: {}", e),
        }
    }
    false
}
