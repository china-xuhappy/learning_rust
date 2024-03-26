// 控制流 https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html
// 最常见的用来控制执行流的结构是 if 表达式和循环
fn control(){
    // if 表达式
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 使用 else if 处理多重条件
    // 使用过多的 else if 表达式会使代码显得杂乱无章  ---- 分支结构match
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 在 let 语句中使用 if
    // 因为 if 是一个表达式，我们可以在 let 语句的右侧使用它
    let condition = true;
    let number = if condition { 5 } else { 6 }; // 像是三元表达式,  类型要一样
    println!("The value of number is: {number}");

    // 使用循环重复执行
    // 有三种循环：loop、while 和 for （continue， break）

    // 使用 loop 重复执行代码
    loop {
        println!("again!");
    }

    // 从循环中 获得结果
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // 循环标签：在多个循环之间消除歧义
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 结束外层的循环
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    // while 条件循环
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");


    // 使用 for 遍历集合
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}