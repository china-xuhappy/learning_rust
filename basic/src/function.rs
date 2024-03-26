// 函数 (https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html)
// 函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。这是一个包含函数定义示例的程序

fn another_function() {
    println!("Another function.");
}

// 参数 : 参数是特殊变量，是函数签名的一部分。当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）
// 必须 声明每个参数的类型
fn another_function(a: i32) {

}

// 多个参数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 语句和表达式
// 语句（Statements）是执行一些操作但不返回值的指令。 表达式（Expressions）计算并产生一个值
fn main() {
    let y = 6; // 语句

    // let y = (let x = 0); //这会报错

    let y = {
        let x = 3;
        x + 1
        // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值
    };  // 这是表达式, 有返回
    println!("The value of y is: {y}");
}

// 具有返回值的函数
fn five() -> i32 {
    5
}
