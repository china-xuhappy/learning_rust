// Slice 类型 https://kaisery.github.io/trpl-zh-cn/ch04-03-slices.html
// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权


fn main(){
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5
    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！



    // 字符串 slice。是 String 中一部分值的引用
    let s = String::from("hello world");

    // 不同于整个 String 的引用，hello 是一个部分 String 的引用， 由一个额外的 [0..5] 部分指定
    let hello = &s[0..5];
    let world = &s[6..11]; // world 将是一个包含指向 s 索引 6 的指针和长度值 5 的 slice。

    let slice = &s[..2]; // 从0开始 可以不写
    let slice = &s[3..]; // 最后一个字节 也可以不写
    let slice = &s[..]; // 俩都不写 是全部字符串


    let mut s = String::from("hello world");
    let word = first_word(&s); // 当拥有某值的不可变引用时，就不能再获取一个可变引用
    s.clear(); // 错误！ clear 需要清空 String，它尝试获取一个可变引用
    println!("the first word is: {}", word);

    // 字符串字面值就是 slice  字符串字面值被储存在二进制文件中
    let s = "Hello, world!";
    // s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。
    // 这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。


    // 字符串 slice 作为参数
    fn first_word(s: &str) -> &str {
        // 它使得可以对 &String 值和 &str 值使用相同的函数：
        // 如果有一个字符串 slice，可以直接传递它。如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用
    
    }

    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);



    // 其他类型的 slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; //  slice 的类型是 &[i32]
    // 通过存储第一个集合元素的引用和一个集合总长度
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.bytes();

    for (i, &item) in bytes.iter().enumerate() { // 因为 enumerate 方法返回一个元组，我们可以使用模式来解构. 获取了集合元素的引用，所以模式中使用了 &
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}