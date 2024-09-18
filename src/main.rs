mod checker;

fn main() {
    // 加载规则文件
    let rules = match checker::load_regex_rules("./rules") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading regex rules: {}", e);
            return;
        }
    };

    // 要检查的文本
    let text = "乳房";

    // 检查文本是否匹配任何正则规则
    if checker::check_text_against_rules(&rules, text) {
        println!("文本匹配了规则");
    } else {
        println!("文本不匹配任何规则");
    }
}
