// 20240321  复习基础知识（变量，数据类型，函数，控制流）
// https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html
fn main() {

    // 变量 可变mut， 不可变无法变更
    let mut var = 5;
    println!("The value of x is: {var}");
    var = 6;
    println!("The value of x is: {var}");

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    // 隐藏(相当于 覆盖)
    let a = 0;
    let a: i32 = a + 2; // 实际上创建了一个新变量
    {
        let a = a * 2;
        println!("The value of x in the inner scope is: {a}");
    }

    println!("The value of x is: {a}");


    // let mut spaces = "   ";
    // spaces = spaces.len(); // 会报错，两个类型不一样


}
