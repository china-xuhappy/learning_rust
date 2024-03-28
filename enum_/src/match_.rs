// match 控制流  https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html
// 它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码
// 模式可由字面值、变量、通配符和许多其他内容构成
fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        // 对于 if，表达式必须返回一个布尔值，而这里它可以是任何类型的

        match coin {
            // 一个模式和一些代码。第一个分支的模式是值 Coin::Penny 而之后的 => 运算符将模式和将要运行的代码分开。这里的代码就仅仅是值 1
            Coin::Penny => 1,
            Coin::Nickel => 5,
            // 如果分支代码较短的话通常不使用大括号
            Coin::Dime => 10,
            // 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
            Coin::Quarter => {
                println!("Lucky penny!");
                25
            },
        }

    }
    
// 绑定值的模式
    #[derive(Debug)] // 这样可以立刻看到州的名称
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => { // 变量 state 将会绑定 25 美分硬币所对应州的值
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

// 匹配 Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

// 匹配是穷尽的
    match x {
        Some(i) => Some(i + 1),
        // 我们没有处理 None 的情况，所以这些代码会造成一个 bug
    }

// 通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //最后一个分支则涵盖了所有其他可能的值，模式是我们命名为 other 的一个变量。
        _ => reroll(),         // Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 _ 
        _ => (), // 不运行任何代码

    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}




}