// 数据类型 https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html

fn data_type(){
// 标量类型
    // 数据类型： 整型、浮点型、布尔类型和字符类型
    // 整型
        // 长度	有符号(有负数)	无符号(都是正数)
        // 8-bit	i8	u8
        // 16-bit	i16	u16
        // 32-bit	i32	u32
        // 64-bit	i64	u64
        // 128-bit	i128	u128
        // arch	isize	usize

    // 浮点型
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 数值运算  加减乘除
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1
    // remainder
    let remainder = 43 % 5;

    // 布尔类型
    let t = true;
    let f: bool = false;

    // 字符类型
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


// 复合类型
    // 元组（tuple）和数组（array）
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构元组
    println!("The value of y is: {y}");

    // 使用 . 点 访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    // 不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。这种值以及对应的类型都写作 ()，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。


    // 数组类型
    // 数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，Rust 中的数组长度是固定的
    // 确定元素个数不会改变时，数组会更有用,  否则用vec
    let a = [1, 2, 3, 4, 5];

    // 月份 固定12个
    let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];

    // 数组是可以在栈 (stack) 上分配的已知固定大小的单个内存块
    let first = a[0];
    let second = a[1];


    // a[10] // 数组越界 无效的访问
}