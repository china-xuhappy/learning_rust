// if let 简单控制流 https://kaisery.github.io/trpl-zh-cn/ch06-03-if-let.html
// 来处理只匹配一个模式的值而忽略其他模式的情况
fn main(){
    let config_max = Some(3u8);
    match config_max {
        // 值并只希望当值为 Some 成员时执行代码
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    // if let 语法获取通过等号分隔的一个模式和一个表达式
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }else {
        // else 块中的代码与 match 表达式中的 _ 分支块中的代码相同
    }


}