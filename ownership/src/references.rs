// 引用 (https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html)

// 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。
//  与指针不同，引用确保指向某个特定类型的有效值。

// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //&s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。
    println!("The length of '{}' is {}.", s1, len);


    
    // 可变引用
    let mut s = String::from("hello");
    change(&mut s); // 传递一个 可变引用

    fn change(some_string: &mut String) { // 接受一个可变引用
        some_string.push_str(", world");
    }

    // 如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用
    let r1 = &mut s;
    let r2 = &mut s; // 报错， 我们不能在同一时间多次将 s 作为可变变量借用，防止同一时间对同一数据存在多个可变引用。

    let mut s = String::from("hello");
    // 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;


    // Rust 在同时使用可变与不可变引用时也采用的类似的规则
    // 我们 也 不能在拥有不可变引用的同时拥有可变引用
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    // 多个不可变引用是可以的

    let r3 = &mut s; // 大问题


    // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
        // 因为最后一次使用不可变引用（println!)，发生在声明可变引用之前
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s; // 没问题
        println!("{}", r3);


    // 悬垂引用（Dangling References）
        // 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者
        fn dangle() -> &String { // dangle 返回一个字符串的引用

            let s = String::from("hello"); // s 是一个新字符串
        
            &s // 返回字符串 s 的引用
        } // 这里 s 离开作用域并被丢弃。其内存被释放。
          // 危险！（这意味着这个引用会指向一个无效的 String）
}

fn calculate_length(s: &String) -> usize { // s 是 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生


// 创建一个引用的行为称为 借用（borrowing）
// 现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。我们并不拥有它。

fn change(some_string: &String) {
    some_string.push_str(", world"); // 不允许修改引用的值
}





//   & 符号就是 引用。   允许你使用值但不获取其所有权
// 与使用 & 引用相反的操作是 解引用（dereferencing）  使用解引用运算符，*



