use core::num;

// 4. 常量可以在任何作用域声明， 包括全局作用域
const MAX_POINTS: u32 = 100_000;

fn variable() {
    // 声明变量 学习 let mut const shadow(隐藏)

    // 不可变的变量
    // let foo = 1; // immutable
    // foo = 2; // 报错


    // 可变的变量 - 变量类型不能变
    // let mut foo = 1;
    // foo = 2; // 正常
    // foo = '2'; // 报错 mismatched types 不匹配的类型

    // 常量 
    // 1. 常量在绑定值以后是不可变的
    // 2. 不可以使用 mut
    // 3. 声明时 类型必须被标注
    // 4. 常量可以在任何作用域声明， 包括全局作用域
    // 5. 无法绑定 函数的调用结果 或者 在运行时算出来的值
    // 6. 在程序运行期间 - 常量在其声明的作用域内 一直有效的
    // 7. 常量使用全大写字母，每个单词之间_下划线隔开
    // println!("MAX_POINTS: {}", MAX_POINTS); // 100000000 而 不是全局 100_000
    // const MAX_POINTS: u32 = 100_000000; // 全局的 名字一样 会覆盖掉全局的
    // println!("MAX_POINTS: {}", MAX_POINTS); // 100000000

    // Shadowing (隐藏)
    // 1. 可以使用相同的名字声明新的变量，新的变量就会shadowing 隐藏之前声明的相同变量名
    let spaces = "           ";
   
    // 2. 如果不适用let关键字，那么重新给非mut的变量赋值会导致编译错误
    // spaces = ""; // cannot assign twice to immutable variable `spaces`

    // 3. 使用let声明 同名的新变量，他的类型可以与之前不同
    let spaces = spaces.len(); // 在后续代码中 这个变量名代表的就是新的变量

    println!("spaces: {}", spaces)
    
}

fn data_type(){
    // 数据类型

    // 两类数据类型子集：标量（scalar）和复合（compound）
    
    // Rust 是静态编译语言，在编译时必须知道所有变量的类型
        // 基于使用的值，编译器通常能够推断出具体类型
        // 当多种类型均有可能时  必须增加类型标注，否则会编译错误
    // let guess = "42".parse().expect("Not a number!"); // type annotations needed
    
    // 把42转为整数， 但是整数表示有很多 i32 u32 等等，所以必须要指明
    let guess: u32 = "42".parse().expect("Not a number!");


    // 标量（scalar）

        // 整数类型
            // 整数类型 没有小数部分
            // 例如 u32 就是一个无符号的整数类型，占据32位的空间
            // 无符号整数类型 以U开头
            // 有符号整数类型 以I开头

            // 整数类型列表
                // 每种都分 i 和 u， 以及固定的位数
                // 有符号范围 : - (2n -1) 到 2n-1  - 1
                // 无符号范围 : 0 到 2n - 1

                // 8-bit      i8      u8 
                // 16-bit     i16     u16
                // 32-bit     i32     u32
                // 64-bit     i64     u64 
                // 128-bit    i128    u128
                // arch       isize   usize

                // arch 是系统的架构
                // isize 和 usize 类型的位数由程序运行的计算机的架构所决定：
                    // 如果是64位计算机, 那就是64位
                    // ...

            // 整数的字面值

                // Decimal 十进制      98_222 可以加 下划线 _ 增加可读性
                // Hex    十六进制     0x 开头 0xff
                // Octal   八进制      0o 开头 0o77
                // Binary  二进制      0b 开头 1111_0000 可以加 下划线
                // Byte(u8 only) 字节类型 仅限于U8  b开头 b 'A'

                // 除了 Byte 类型外，所有的数值字面值都允许使用类型后缀
                    // 例如 57u8  57是一个整数 u8 就是类型
                
                // 如果不清楚应该使用哪种类型，可以使用Rust相应的默认类型：
                    // 整数默认类型是 i32
                        // 总体来说速度很快，即使在64位系统中
            
            // 整数溢出
                // 例如: u8 的范围是 0 - 255，如果你把一个u8变量的值设为256，那么: 
                    // 1. 调试模式下编译：Rust 会检查整数的溢出，如果发生溢出，程序在运行时就会panic
                    // 2. 发布模式下 (--release) 编译：Rust 不会检查可能导致的 panic 的整数溢出
                    
                    // 如果溢出发生：Rust 会执行 "环绕" 操作：
                        // 256 变成 0， 257 变成 1 ................
        
        // 浮点类型
            // Rust 有两种基础的浮点类型，也就是含有小数部分的类型
                // f32, 32位，单精度
                // f64, 64位，双精度
            
            // Rust 的浮点类型使用了 IEEE-754 标准来表述

            // f64 是默认类型，因为在现代CPU上 f64 和 f32的速度 差不多，而且经度更高
                // let x = 2.0; // 默认 f64
                // let y: f32 = 3.0;

            // 数值的操作
                // 加减乘除余 等(例子)

                let sum = 5 + 10;

                let difference = 95.5 - 4.3;

                let product = 4 * 30;

                let quotient = 56.7 / 32.2;

                let reminder = 54 % 5;

        // 布尔类型
            // Rust 的布尔类型也有俩值：true 和 false
            // 一个字节的大小
            // 符号是bool
            // 例子:
                let t = true;
                let f: bool = false;

        // 字符类型
            // Rust 语言中 char 类型被用来描述语言中最基础的单个字符。
            // 字符类型的字面值 使用单个引号
            // 占用4字节大小
            // 是Unicode 标量值，可以表示比ASCII多得多的字符内容： 拼音、中日韩文、零长度空字符、emoji 表情等
                // U+0000 到 U+D7FF
                // U+E000 到 U+10FFFF

            let x = 'z';

            let y:char = 'y';

    // 复合类型
        // 复合类型可以将多个值放在一个类型
        // Rust 提供了俩基础的复合类型：元组 (Typle) 、 数组

        // Tuple
            // Tuple 可以将多个类型 多个值放在一个类型里面
            // Tuple 的长度是固定的：一旦声明就无法改变

            // 在小括号里，将值用逗号分开
            // Tuple 中的每一个位置都对应一个类型，Tuple 中各元素的类型不必相同

            let tup: (i32, f64, u8) = (500, 6.4, 1);
            

            // 获取 Tuple 的元素值
            // 可以使用 默认匹配来解构（destructure）一个Tuple来获取元素的值

            let (x, y, z) = tup; // 解构赋值


            // 访问 Tuple 的元素
            // 在Tuple 变量使用点标记法，后接元素的索引号
            println!("{}, {}, {}", tup.0, tup.1, tup.2);

        // 数组
            // 数组也可以将多个值 放在一个类型里面
            // 数组中每个元素的类型必须相同
            // 数组的长度也是固定的

            // 在中括号里，将值用逗号分开

            let a = [1,2,3,4,5,6,7,8,9];

            // 数组的用处
                // 如果想让你的数据存放在stack（栈）上而不是heap（堆）上，或者想保住有固定数量的元素，这时使用数组更有好处
                // 数组没有Vector灵活
                    // Vector 和 数组类似，它由标准库提供
                    // Vector 的长度可以改变
                    // 如果你不确定应该用数组还是Vector，那么估计你应该用Vector
                
                let months = [
                    "January",
                    "February",
                    "March",
                    "April"
                ]; // 所有月份

            // 数组的类型
                // 数组的类型 以这种形式表示：[类型; 长度]

                let a: [i32; 5] = [1, 2, 3, 4, 5];

            
            // 如果数组的每个元素值都是相同，那么可以在：
                // 在中括号里指定初始值
                // 然后是一个 ;
                // 最后是数组的长度

                let b = [3; 5]; // 相当于 let b = [3,3,3,3,3];

            // 访问数组的元素

                // 数组是在Stack（栈）上分配的单个块的内存
                // 可以使用索引来访问数组的元素（例子）
                let first = months[0]; // January
                let february = months[1];

                println!("first: {}", first);

                // 如果访问的索引超出了数组的范围，那么：
                    // 编译时会通过 -- 也会有一些简单的检查
                    let index = 4;
                    // this operation will panic at runtime
                    // index out of bounds: the length is 4 but the index is 4
                    // let month = months[index]; // 报错

                    // 运行时会报错 （runtime 和 panic）
                        // Rust 不会允许其继续访问相应（超出）地址的内存
                        let index_arr = [3,4,5];
                        // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 5', src\main.rs:228:37
                        // let month = months[index_arr[2]]; // 运行时报错
}

fn five(x: i32) -> i32 {
    x + 5 * 2
}

fn another_function(x: i32, y: u32) { // parameters
    println!("i {}, y {}", x, y);
}

fn function(){
    // 声明函数使用 fn 关键字
    // 依照惯例，针对函数和变量名，Rust 使用snake case命名规范：
        // 所有的字母都是小写的，单词之间使用下划线_分开
        // another_function()

    // 函数的参数
        // parameters 定义的参数  形参， arguments 传的参数，实参
        // 在函数签名里，必须声明每个参数的类型
        another_function(5, 6);

    // 函数体中的语句与表达式
        // 函数体由一系列语句组成，可选的由一个表达式结束
        // Rust 是一个基于表达式的语言
        // 语句是执行一些动作的指令
        // 表达式会计算产生一个值 - 本身就是一个值
        let a = 5 + 6; // 5 和 6 就是表达式  计算出来的11也是。 赋值给a 

        // 函数的定义也是语句
        

        // 语句不返回值，所以不可以使用let将一个语句赋值给一个变量
        // 等号后面 应该是表达式， 但是发现右边是个语句
        // let x = (let y = 6); //expected expression, found statement (`let`)

        // 调用函数是表达式，调用宏也是表达式


    // 函数的返回值

        // 在 -> 符号后面声明函数返回值的类型，但是 不可以为返回值命名
        // 在Rust里面，返回值就是函数体里面最后一个表达式的值
        // 若想提前返回，需使用return关键字，并指定一个值
            // 大多数函数都是默认使用最后一个表达式 做为返回值

            let x = five(6);
            println!("five()  x: {}", x);

}

fn control() {
    // if 表达式
        // if 表达式允许你 根据条件来执行 不同的代码分支
            // 这个条件必须是 bool 类型
        
        // if 表达式中，与条件相关联的代码块就叫做分支 （arm）
        // 可选的，在后边可以加一个 else 表达式

        let number = 3;

        if number < 5 {
            println!("true");
        } else {
            println!("false")
        }

        // 使用 else if 处理多重条件
        // 但如果使用了多于一个 else if ，那么最好使用 match 来重构代码
        let number = 6;
        if number % 4 == 0 {
            println!("4");
        }else if number % 3 == 0 {
            println!("3");
        }else if number % 2 == 0 {
            println!("2");
        }else {
            println!("else");
        }

        // 在 let 语句中使用if
            // 因为 if 是一个表达式， 所以可以将它放在let语句中 等号的右边

                let condition = true;

                // 在编译的时候 必须知道类型是什么， 所以结果类型不能不同
                // `if` and `else` have incompatible types
                // let number = if condition {5} else {"6"}; 
                let number = if condition {5} else {6};

                println!("the value of number is: {}", number);
    // Rust 的循环
        // loop ，while，和 for

        // loop
            // loop 关键字告诉Rust 反复的执行一块代码，直到你喊停为止

            // loop {
            //     println!("again!")
            // } // Ctrl + C 结束
            
            let mut counter = 0;
            
            // 可以在 loop 循环中使用break 关键字 来告诉程序何时停止循环 并返回结果
            // break
            let result = loop {
                counter += 1;

                if counter == 10 {
                    break counter * 2;
                }
            };

            println!("the result is: {}", result);
        // while 条件循环
            // 另一种常见的循环模式 是 每次执行循环体之前都判断一次条件。
            // while 条件循环为这种模式而生
            let mut number = 3;

            while number != 0 {

                println!("{}", number);
                number = number - 1;
            }

            println!("Leftoff!!");
        
        // 使用for循环 遍历集合
            // 针对集合中的每一个元素来执行一些代码

            let a = [10,20,30,40,50];
            for element in a.iter() {
                println!("the value is: {}", element);
            }

            // Range (1..4) -- 和Python range 差不多
            // 标准库 提供的， 指定一个开始数字 和 一个结束数字，range 可以生成它们之间的数字 （不含结束）
            // rev 方法 可以 反转 Range
            for i in (1..4).rev() {
                println!("the value is: {}",i);
            }

}

fn ownership(){
    // 所有权
    // 所有权是Rust最独特的特效，它让Rust 无需GC(垃圾回收机制)就可以保证内存安全。
        // 4.1 什么是所有权
        // Rust 的核心特点就是所有权
        // 所有程序在运行时都必须管理它们使用计算机内存的方式
            // 有些语言有垃圾回收机制，在程序运行时，它们会不断的寻找不再使用的内存 Java 
            // 在其他语言中，程序员必须显式的分配和释放内存   C || C++
        // Rust 采用了第三种方式：
            // 编译器在编译时会根据一系列的规则进行检查。
            // 如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。

    // Stack vs Heap
    // 栈内存 vs 堆内存
        // 在像Rust 这样的系统级别的语言里，一个值是在 stack 上还是在heap上 对语言的行为和你为什么要做某些觉得有更大的影响
        // 在代码运行的时候，stack 和 heap 都是可以用的内存，但他们的结构很不相同

        // 栈
            // 后进先出（last in, first out）。
                // 添加数据 叫做 压栈
                // 移除数据 叫做 出栈
            // 想象一下一叠盘子：当增加更多盘子时，把它们放在盘子堆的顶部，当需要盘子时，也从顶部拿走。不能从中间也不能从底部增加或拿走盘子！
            
            // 栈中的所有数据都必须占用已知且固定的大小。
                // 在编译时大小未知或大小可能变化的数据，要改为存储在堆上

        // 堆

            // 把数据放在 heap 时，会请求一定数量的空间
            // 操作系统在 heap 里找到一块足够大的空间，把它标记为在用，并返回一个指针，也就是这个空间的地址
            // 这个过程叫做 在 heap 上进行分配，有时仅仅称为 “分配”

        // Stack vs Heap
        // 存储数据
            // 把值压到Stack上 不叫分配

            // 因为指针是已知固定大小的，可以把指针存放在 Stack 上。
                // 但如果想要实际数据， 你必须使用指针来定位。

            // 把数据压到 Stack 上，要比在 Heap 上分配快的多。
                // 因为操作系统 不需要寻找用来存储新数据的空间，那个位置永远都在Stack的顶端
            
            // 在 Heap 上分配空间需要做更多的工作
                // 操作系统首先需要找到一个足够大的空间来存放数据，然后要做好记录方便下次分配

        // 访问数据
            // 访问Heap中的数据要比访问Stack中的数据慢，因为需要通过指针才能找到 Heap 中的数据。
                // 对于现代的处理器来说，由于缓存的缘故，如果指令在内存中跳转的次数越少，那么速度就越快
                
                // 假设：有一个服务员在餐厅里处理多个桌子的点菜。在一个桌子报完所有菜后再移动到下一个桌子是最有效率的。
                // 从桌子 A 听一个菜，接着桌子 B 听一个菜，然后再桌子 A，然后再桌子 B 这样的流程会更加缓慢

            // 如果数据存放的距离比较近，那么处理器的处理速度就会更快一些 （放在 Stack 上面）
            // 如果数据之间的距离比较远，那么处理速度就会慢一些 （Heap 上）
                // 在 Heap 上分配大量的空间 也是需要时间的

        // 函数调用
            // 当你的代码调用一个函数时，传递给函数的值（包括可能指向堆 (Heap) 上数据的指针）和函数的局部变量被压入栈（Stack）中。当函数结束时，这些值被移出栈（Stack）。

        // 所有权存在的原因

            // 所有权解决的问题：
                // 跟踪代码的那些部分 正在使用 Heap 的那些数据
                // 最小化 Heap 上的重复数据量
                // 清理 Heap 上未使用的数据 以避免空间不足

            // 一但你懂得了所有权，那么就不需要经常去想 Stack 和 Heap了。
            // 但是知道管理 Heap 数据 是所有权存在的原因，这有助于解释它为什么会这样工作。

    // 所有权的规则
        // 每个值都有一个变量，这个变量是该值的所有者
        // 每个值同时 只能有一个所有者
        // 当所有者超出作用域 （Scope）时，该值将被删除。

        // 变量作用域
            // Scope 就是程序中一个项目的有效范围

            fn scope () {
                // s 还没定义 不可以
                let s = "hello"; // s 可用

                println!("{}", s);  // 可用对 s 进行相关的操作
            };
            // s 作用域到此结束，s 不在可用

            // cannot find value `s` in this scope   在此范围内找不到值`s`
            // println!("{}", s); 
        
        // String 类型
            // String 比那些基础标量数据类型更复杂
            // 字符串字面值：程序里手写的那些字符串值。它们是不可改变的  --- 不是所有字面值 在编译时可用确认
            // Rust 还有第二种字符串类型：String
                // 在 Heap 上分配, 能够存储在编译时未知数量的文本.

            // 创建String 类型的值
                // 可用使用 from 函数从字符串字面值创建出 String 类型
                // let s = String::from("hello");
                    // "::" 表示 from 是 String 类型下面的函数
                // 这类字符串是可以被修改的
                let mut s = String::from("hello");
                s.push_str(", world");
                println!("{}", s); // hello, world

                // 为什么 String 类型的值可以修改，而字符串字面值却不能修改？
                    // 因为他们处理的内存的方式不同

            // 内存和分配
                // 字符串字面值，在编译时就知道它的内容了， 其文本内容直接被硬编码到最终的可执行文件里
                    // 速度快、高效。 是因为其不可改变。

                // String 类型，为了支持可变性，需要在Heap上分配内存来保存编译时未知的文本内容：
                    // 操作系统必须在运行时来请求内存
                        // 这步通过调用 String::from 来实现
                    // 当用完String之后，需要使用某种方式将内存返回给操作系统
                        // 这步，在拥有GC的语言中，GC会跟踪并清理不再使用的内存
                        // 没有GC，就需要我们去识别内存何时不再使用，并调用代码将它返回。
                            // 如果忘了，那就浪费内存。
                            // 如果提前做了，变量就会非法
                            // 如果做了两次，也是BUG，必须一次分配对应一次释放

                // Rust 采用了 不同的方式：对于某个值来说，当拥有它的变量走出作用范围时，内存会立即自动的交还给操作系统

                fn drop_string() {
                    let mut s = String::from("hello");
                    s.push_str(", world");
                    println!("{}", s); // hello, world
                } // 变量离开作用域，Rust 调用 Drop 函数

                // drop 函数

            // 变量和数据交互的方式：移动（move）
                // 多个变量可以与同一个数据使用一种独特的方式来交互

                let x = 5;
                let y = x;
                // 整数是已知且固定大小的简单的值， 这俩个5被压到了 Stack 中
            
            // 变量和数据交互的方式：移动（move） String 版本
                let s1 = String::from("hello");
                // 一个String 由3部分组成：
                    // 一个指向存放字符串内容的内存的指针  ptr
                    // 一个长度   len
                    // 一个容量   capacity
                        //      s1 (stack)                                   栈内存(heap)
                        // name  value                                  index    value
                        // ptr      存放内容的内存指针  ----------->         0        h
                        // len      5                                      1        e
                        // capacity 5                                      2        l
                        //                                                 3        l
                        //                                                 4        o
                    // 长度 len，就是存放字符串内容所需的字节数
                    // 容量 capacity 是指 String 从操作系统总共获取内存的总字节数

                // 当把 s1 赋值给 s2，String 的数据被复制了一份：
                let s2 = s1;
                    // 在Stack 上复制了一份指针、长度、容量
                    // 并没有复制指针所指向的 heap 上的数据
                        //      s1 (stack)                                   
                        // name  value                                  
                        // ptr      存放内容的内存(Heap)指针  ----------->         
                        // len      5                                      
                        // capacity 5                                         栈内存(heap)
                        //                                                  index    value  
                                                                            // 0        h
                                                                            // 1        e
                                                                            // 2        l
                        //      s2 (stack)                                    3        l
                        // name  value                                         4        o
                        // ptr      存放内容的内存(Heap)指针  ----------->     
                        // len      5                                     
                        // capacity 5     
                //  当变量离开作用域时，Rust 会自动调用 Drop 函数，并将变量使用的 Heap 内存释放
                // 当 S1 、 S2 离开作用域时，它们都会尝试 释放相同的内存
                    // 二次释放 （double free） Bug
                
                // 为了保护内存的安全：
                    // Rust 没有尝试复制被分配的内存
                    // Rust 让 S1 失效。
                        // 当 S1 离开作用域的时候，Rust 不需要释放任何东西
                    
                    // 试试看当 s2 创建后，再使用 s1 (例子)

                        let s1 = String::from("hello");
                        let s2 = s1; // s1 失效

                        // borrow of moved value: `s1`
                        // value borrowed here after move
                        // 发生了移动， s1 移动给了 s2
                        // println!("s1, {}", s1);
                
                // 浅拷贝 （shallow copy ）
                // 深拷贝 （deep copy）
                // 你也许会将复制指针，长度，容量视为浅拷贝，但由于 Rust 让 s1 失效了，所以我们用一个新的术语 ：移动（move）
                    //     s1 (stack)  (失效)                                 
                    // name  value                                  
                    // ptr      存放内容的内存(Heap)指针  ----------->         
                    // len      5                                      
                    // capacity 5                                         栈内存(heap)
                    //                                                  index    value  
                                                                        // 0        h
                                                                        // 1        e
                                                                        // 2        l
                    //      s2 (stack)  (有效)                             3        l
                    // name  value                                         4        o
                    // ptr      存放内容的内存(Heap)指针  ----------->     
                    // len      5                                     
                    // capacity 5  
                // 隐含的一个设计原则： Rust 不会自动创建数据的深拷贝
                    // 就运行时性能而言，任何自动赋值的操作都是廉价的

            // 变量和数据交互的方式：克隆 clone()
                // 如果真想对 heap 上面的String 数据进行深度拷贝，而不仅仅是Stack上的数据，可以使用clone方法
                    let s1 = String::from("hello");
                    let s2 = s1.clone();

                    println!("s1 {}, s2 {}", s1 , s2);
                    //      s1 (stack)                                   栈内存(heap)
                    // name  value                                  index    value
                    // ptr      存放内容的内存指针  ----------->         0        h
                    // len      5                                      1        e
                    // capacity 5                                      2        l
                    //                                                 3        l
                    //                                                 4        o

                    //      s2 (stack)                                   栈内存(heap)
                    // name  value                                  index    value
                    // ptr      存放内容的内存指针  ----------->         0        h
                    // len      5                                      1        e
                    // capacity 5                                      2        l
                    //                                                 3        l
                    //                                                 4        o

            // Stack 上的数据 复制
                let x = 5;
                let y = x;

                println!("{}, {}", x, y);

                // Copy trait, 可以用于像整数这样完全存放在 Stack 上面的类型
                // 如果一个类型实现了 Copy 这个 Trait， 那么旧的变量赋值后仍然可用

                // 如果一个类型或者该类型的一部分实现了 Drop trait ，那么 Rust 不允许让它再去实现 Copy trait 了

            // 一些拥有 Copy trait 的类型
                // 任何简单标量的组合类型 都可以是Copy的
                // 任何需要分配内存或某种资源都不是 Copy 的
                // 一些拥有Copy trait 的类型
                    // 所有的整数类型,例如 u32
                    // bool
                    // char
                    // 所有的浮点类型, 例如 f64
                    // Tuple (元组)， 如果其所有的字段都是 Copy 的
                        // {i32, i32} 是
                        // {i32, String} 不是
        // 所有权 与 函数
            // 在语义上，将值传递给函数和把值赋给变量是类似的：
                // 将值传递给函数，将发生移动或复制。
                    let s = String::from("hello world");

                    take_ownership(s);  // s 的值 移动到函数里面， 

                    // borrow of moved value: `s`
                    // value borrowed here after move
                    // 从这以后 s 不再有效
                    // println!("s after {}", s);

                    let x = 5;

                    makes_copy(x);

                    println!("x {}", x); // x 5

                    fn take_ownership(some_string : String){
                        println!("take_ownership some_string {}", some_string); // take_ownership some_string hello world
                    } // drop some_string

                    fn makes_copy (some_number: i32){
                        println!("makes_copy some_number {}", some_number); // makes_copy some_number 5
                    } // stack 不会发生什么事情
                
            // 返回值 与 作用域
                // 函数在返回值的过程中 同样也会发生所有权的转移
                    let s1 = gives_ownership();

                    let s2 = String::from("hello");

                    let s3 = takes_and_gives_back(s2); // 把 s2 移动到函数里面

                    // s2 已经移动给函数了， 最后只需要drop s1 和 s3

                    fn gives_ownership() -> String {
                        let some_string = String::from("hello");
                        some_string
                    } // 返回值 所有权 移动 ： 把 some_string 移动给 s1 

                    fn takes_and_gives_back(a_string: String) -> String{
                        a_string
                    } // a_string 作为返回值，移动到 调用函数的变量 - 移动到 s3
                // 一个变量的所有权总是遵循同样的模式：
                    // 把一个值赋给其它变量时 就会发生移动.
                    // 当一个包含 heap 数据的变量离开作用域时, 它的值就会被drop函数清理, 除非数据的所有权移动至另一个变量上了.
                
            // 如何让函数使用某个值， 但不获取所有权？
                // 这种方法 太笨了
                    let s1 = String::from("hello");
                    
                    let (s2 , len) = calculate_length(s1);

                    println!("s2 {}, s2 len {}", s2, len); // s2 hello, s2 len 5
                    
                    fn calculate_length(s: String) -> (String , usize) {
                        let len = s.len();

                        (s, len)
                    } // s 的所有权 返回， 就交给了 s2 接受变量
                // Rust 有一个特性 叫做 “引用 （reference）”
}       

fn reference() {
    // 引用 和 借用
        let s1 = String::from("hello");

        // 把s1的引用，传递给这个函数  & 标识传递的是引用
        // &s1 它并没有 s1 当它走出作用域时，他指向的值s1 并不会被清理
        let len = calculate_length(&s1);

        println!("s1, {}, len , {}",s1 , len); // len , 5

        // 参数 接受一个 &String 类型的引用               以引用作为参数的行为，我们叫做借用。
        fn calculate_length(s : &String) -> usize {
            s.len()
            // 引用 我们并没有获得所有权， 所以不用考虑 把这个值返回回去
        }
        // 参数的类型是 &String 而不是 String

        // &符号就标识引用：允许你引用某些值而不取得所有权

            // s 是 s1的引用，实际上 s 是一个指针 指向了 s1 也是一个指针 指向了 heap 上面的内容
            //      s (引用)                s1 (stack)                                   栈内存(heap)
            //   name  value              name  value                                  index    value
            //    ptr     ------>>        ptr      存放内容的内存指针  ----------->         0        h
            //                            len      5                                      1        e
            //                            capacity 5                                      2        l
            //                                                                            3        l
            //                                                                            4        o

    // 借用
        // 我们把引用作为函数参数这个行为 叫做借用
        // 是否可以修改借用的东西？ --- 不可以  
            fn calculate_length_2(s : &String) -> usize {
                // cannot borrow `*s` as mutable, as it is behind a `&` reference
                // 不可以把借用的变量 作为可变的
                // s.push_str("world");
                s.len()
                // 引用 我们并没有获得所有权， 所以不用考虑 把这个值返回回去
            }
        // 和变量一样，引用默认也是不可变的
    
    // 可变引用

        let mut s1 = String::from("hello"); // 可变得属性
        let len = calculate_length_3(&mut s1); // 引用可变的参数

        println!("s1, {}, len , {}",s1 , len); // len , 5

        // 函数接受参数，&mut String类型可变的引用
        fn calculate_length_3(s : &mut String) -> usize {
            s.push_str("world");
            s.len()
        }

        // 可变引用有一个特殊的限制：在特定作用域内，对某一块数据，只能有一个可变的引用。
            // 这样做的好处，就是在编译的时候防止数据竞争
        let mut s = String::from("hello");
        
        let s1 = &mut s;

        // cannot borrow `s` as mutable more than once at a time
        // 如果把s变成 可变引用，可变引用的个数 不能超过1个
        // let s2 = &mut s; // 报错
        // println!("s1 {}, s2 {}", s1, s2);

        // 以下有三种行为会发生数据竞争
            // 两个或者多个指针同时访问同一个数据
            // 至少有一个指针可用于写入数据
            // 没有使用任何机制来同步数据的访问
        
        // 可以通过创建新的作用域，来允许非同时的创建多个可变引用。
            let mut s = String::from("hello");
            {
                let s1 = &mut s;
            }
            let s2 = &mut s;
        
        // 另外一个限制
            // 不可以同时拥有一个可变引用和一个不可变引用
            let mut s = String::from("hello");
            let r1 = &s;
            let r2 = &s;

            // cannot borrow `s` as mutable because it is also borrowed as immutable
            // 不可以把s 变为可变的引用， 因为他已经是不可变的引用
            // let s1 = &mut s; // 报错 -- 不可以有不可变 和 可变的引用
            // println!("s1, {}, r1, {}, r2 {}", s1, r1, r2);

            // 多个不可变的引用是可以的
            let r1 = &s;
            let r2 = &s;
        
        // 悬空引用 Dangling References
            // 悬空指针（Dangling Pointer）：一个指针引用了内存中的某个地址，而这块内存可能已经被释放并分配给其它人使用了。

            // 在 Rust 里，编译器可保证引用永远都不是悬空引用：
                // 如果你引用了某些数据，编译器将保证在引用离开作用域之前数据不会离开作用域

                // missing lifetime specifier
                // fn fangle() -> &String {
                //     let s = String::from("hello");
                //     &s
                // } // s 这个值离开作用域 将被销毁
                // 返回引用 报错，已经释放的内存地址，因为Stack 和 Heap 中的数据都已经被销毁了
        
        // 引用的规则
            // 在任何给定的时刻，只能满足以下条件之一
                // 一个可变的引用
                // 任意数量不可变的引用
            // 引用必须一直有效

            
}

fn slice() {
    // 切片
        // Rust 的另一种不持有所有权的数据类型：切片（slice）
        // 一道题，编写一个函数：
            // 它接受字符串作为参数
            // 返回它在这个字符串里找到的第一个单词
            // 如果函数没找到任何空格，那么整个字符串就被返回。
        
            let mut s = String::from("hello");

            // 如果这个索引位置， 脱离了字符串的上下文 那就毫无意义
            // 在函数反复以后 就无法保证 s 字符串的有效性 （不变动）
            let worldIndex = first_world(&s); // 5 
        
            s.clear(); // 可以清空, 那么索引（worldIndex）位置 还是5 ， 就毫无意义
            // 解决这个问题 使用 字符串切片
            println!("worldIndex, {}", worldIndex);
            
            fn first_world(s: &String) -> usize{
                let bytes = s.as_bytes();

                for (index, &item) in bytes.iter().enumerate() {
                    if item == b' ' {
                        return index;
                    }
                }
                s.len()
            }
        // 字符串切片
            // 字符串切片是指向字符串中一部分内容的引用
                let s = String::from("hello world");

                // &s 对s 的引用， [0..5] 引用字符串中的一部分
                let hello = &s[0..5];
                let world = &s[6..11];

                println!("{}, {}", hello, world); // hello, world
            
            // 形式：&对变量的引用，[开始索引..结束索引]

                // 开始索引就是切片起始位置的索引值
                // 结束索引是切片终止位置的下一个索引值
                    //      s (stack)                                   栈内存(heap)
                    // name  value                                  index    value
                    // ptr      存放内容的内存指针  ----------->         0        h
                    // len      11                                     1        e
                    // capacity 11                                     2        l
                    //                                                 3        l
                    //                                                 4        o
                                                                    // 5        
                    //      world [6..11]                                   
                    // name  value                                  
                    // ptr      指针从6 到 11  ----------->             6        w
                    // len      5                                      7        o
                    //                                                 8        e
                    //                                                 9        l
                    //                                                 10       d
                    //                                                 
            // 语法糖
                // 从前面开始 0 可以不写
                let hello = &s[..5];

                // 到末尾结束，后面也可以不写
                let world = &s[6..];

                // 所有字符串的切片 前后都不写
                let whole = &s[..];

                println!("{}, {}", hello, world); // hello, world
        // 注意
            // 字符串切片的范围索引必须发送在有效的 Utf-8 字符串边界内。
            // 如果尝试从一个多字节的字符串创建字符串切片，程序会报错并退出

        // 用切片 修改之前例子
            let mut s = String::from("hello");

            let worldIndex = first_world_2(&s); // 5 
        
            // s.clear(); // 不可以把s 借用为可变的，因为它已经是不可变的了
            println!("worldIndex first_world_2, {}", worldIndex); // worldIndex first_world_2, hello
            
            fn first_world_2(s: &String) -> &str{ // &str 字符串切片
                let bytes = s.as_bytes();

                for (index, &item) in bytes.iter().enumerate() {
                    if item == b' ' {
                        return &s[..index];
                    }
                }
                &s[..]
            }

        // 字符串字面值是切片
            // 字符串字面值被直接存储在二进制程序中

            // s: &str = 它就是指向二进制程序 特定位置的一个切片
            // &str 是不可变的引用 ，所以字符串字面值 是不可变的
            let s = "Hello World";

            // 变量s的类型是&str，它是一个指向二进制程序特定位置的切片
                // &str 是不可变的引用，所以字符串字面值也是不可变的

        // 字符串切片作为参数传递
            // fn first_world(s: &String) -> &str {}
            // 有经验的rust 开发者会采用 &str 作为参数类型，因为这样就可以同时接受 String 和 &str类型的参数了。
            // fn first_world(s: &str) -> &str {}
                // 使用字符串切片，直接调用该函数
                // 使用String，可以创建一个完整的String 切片来调用该函数
                // 也可以直接使用String
            
            // 定义函数时可以使用字符串切片来代替字符串引用会使我们APi 更加通用，且不会损失任何功能。

            let mut s = String::from("hello");
            let worldIndex = first_world_3(&s);
            println!("world index 1{}", worldIndex); // world index1 hello 

            let worldIndex = first_world_3(&s[..]);
            println!("world index 2{}", worldIndex); // world index2 hello
            
            let s = "Hello World";
            let worldIndex = first_world_3(s);
            println!("world index 3{}", worldIndex); // world index 3Hello
            fn first_world_3(s: &str) -> &str{ // &str 字符串切片
                let bytes = s.as_bytes();

                for (index, &item) in bytes.iter().enumerate() {
                    if item == b' ' {
                        return &s[..index];
                    }
                }
                &s[..]
            }
    // 其他类型的切片
        let a = [1, 2, 3, 4, 5];
        
        // slice 切片 就是在Stack 存储了一个指针指向了一个Heap中起始位置 到结束位置的一个指针
        // 还有个长度 是2

        let slice = &a[1..3];
}

fn struct_(){
    // struct 结构体  --- json 差不多
        // 自定义的数据类型
        // 为相关联的值命名， 打包 => 有意义的组合

    // 定义 struct
        // 使用 struct 关键字，并为整个 struct 命名
        // 在花括号内，为所有字段（field）定义名称和类型

        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }
    // 实例化 struct
        // 需要创建 struct 的实例：
            // 为每个字段指定具体值
            // 无需按声明的顺序进行指定
        
        // 注意：一旦 struct 的实例是可变的，那么实例中所有的字段都是可变的
        let mut user1 = User {
            email: String::from("xuhappy@qq.com"),
            username: String::from("xuhappy"),
            active: true,
            sign_in_count: 1,
        }; // 必须要和struct 对应字段复制，不能少 不能多
    
    // 获取 struct 某个值
        // 使用 . 点标记法
        user1.active = false;
        println!("username, {}", user1.username);
    
    // struct 可以作为函数的返回值
        fn build_user (email: String, username: String) -> User {
            User { 
                username: username, 
                email: email, 
                sign_in_count: 1, 
                active: true
            }
        };
    // 字段初始化简写
        // 当字段名与字段值对应变量名相同时，就可以使用字段初始化简写的方式
        fn build_user_1 (email: String, username: String) -> User {
            User { 
                username, 
                email, 
                sign_in_count: 1, 
                active: true
            }
        };
    // struct 更新语法
        // 当你想基于某个struct 实例来创建 一个新实例的时候，可以使用 struct 更新语法：
        let user2 = User {
            username: String::from("xuhappy222"),
            email: String::from("111@qq.com"),
            active: user1.active,
            sign_in_count: user1.sign_in_count,
        };
        // 如果 username email 这俩需要自己声明 正常写， active sign_in_count 而外的这俩需要 user1 的属性 就不用显示的写，可以使用一些方法
        // user2 还没赋值的字段 都会把user1 所以字段 给 user2 
        let user2 = User {
            username: String::from("xuhappy222"),
            email: String::from("111@qq.com"),
            ..user1
        };
    
    // Tuple struct
        // 可以定义类似 tuple 的 struct ，叫做 tuple struct 
            // tuple struct 整体有个名，但里面的元素没有名
            // 适用：想给整个tuple 起名，并让它不同于 其它 tuple ，而且又不需要给每个元素起名
        // 定义 tuple struct ：适用 struct 关键字，后边是名字，以及里面元素的类型

        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        // black 和 origin 是不同的类型，是不同 tuple struct 的实例。
    
    // unit-like struct （没有任何字段）
        // 可以定义没有任何字段的 struct ，叫做 unit-like struct （因为与() , 单元类型类似）
        // 适用于需要在某个类型上实现某个 trait，但是在里面又没有想要存储的数据

    // struct 数据的所有权
        struct UserInfo {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }
        // 这里的字段使用了 String 而不是 &str:
            let userinfo_1 = UserInfo {username: String::from("xuhappy"), email: String::from("123@qq.com"), sign_in_count: 2, active: false,};
            // 该struct实例拥有其所有的数据
            // 只要struct 实例是有效的，那么里面的字段数据也是有效的
        // struct 里也可以存放引用，但这需要使用生命周期
            // 生命周期保证只要struct实例是有效的，那么里面的引用也是有效的.
            // struct UserInfo_2 {
            //     // missing lifetime specifier
            //     // 没有指定生命周期
            //     username: &str,
            //     email: String,
            //     sign_in_count: u64,
            //     active: bool,
            // }
    // 例子： 计算长方形面积
        #[derive(Debug)] //注解：对struct 显示的选择 debug 打印功能
        struct Rectangle {
            width: u32,
            height: u32,
        }      
        
        let rect = Rectangle {
            width: 100,
            height: 20,
        };

        println!("{}", calculate_rectangle(&rect)); // 2000

        /**
         * Rectangle {
            width: 100,
            height: 20,
        }
         */
        // "{}" // rust 没有对 struct 实现Display接口
        println!("{:#?}", rect); // {:?} {:#?} 打印 struct 

        fn calculate_rectangle(rect: &Rectangle) -> u32{
            rect.width * rect.height
        }
    // struct 里面定义方法
        // 方法和函数类似：fn 关键字， 名称，参数，返回值
        // 方法与函数不同之处：
            // 方法是在 struct (或 enum, trait对象) 的上下文中定义
            // 第一个参数是self，表示方法被调用的struct 实例
    
        // 定义方法
            // 在impl 块里定义方法
            // 方法第一个参数可以是 &self (借用)， 也可以获得其所有全，可变借用（&mut self）。是其它参数一样
            // 更良好的语言组织
            impl Rectangle {
                fn calculate(&self) -> u32 {
                    self.height * self.width
                }
            }

            let rect = Rectangle {
                width: 100,
                height: 20,
            };
            
            println!("{}", rect.calculate()); // 2000
        // 方法调用的运算符
            // C / C++: object -> something() 和 (*object).something() 一样
            // Rust 没有 -> 运算符
            // Rust 会自动引用或解引用
                // 在调用方法时 就 会发生这种行为
            // 在调用方法时，Rust 根据情况自动添加 &，&mut（可变引用） 或 *(解引用)，以便object 可以匹配方法的签名
            
            // 下面两行代码效果相同：
                // p1.distance(&p2);
                // (&p1).distance(&p2);

        // 方法参数
            // 方法可以有多个参数
            impl Rectangle{
                fn can_hold(&self, other: &Rectangle) -> bool{
                    self.width > other.width && self.height > other.height
                }
            }

            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };

            let rect2 = Rectangle {
                width: 10,
                height: 20,
            };

            println!("{}", rect1.can_hold(&rect2)); // true
        // 关联函数
            // 可以在impl块里定义不把self作为第一个参数的函数，它叫做关联函数（不是方法）
                // 例如：String::from() 关联函数
            // 关联函数通常用于构造器, 创建被关联类型的实例（例子）

            impl Rectangle {
                fn square(size: u32) -> Rectangle {
                    Rectangle { width: size, height: size }
                }
            }

            let square = Rectangle::square(20);
            println!("{:#?}", square);

            // ::符号
                // 关联函数
                // 模块创建的命名空间

            // 多个 impl 块
                // 每个struct 允许拥有多个 impl 块

}

fn enum_(){
    // 枚举
        // 枚举允许我们列举所有可能的值来定义一个类型

        // 定义枚举
            // IP地址：IPv4，IPv6
            enum IpAddrKind {
                V4,
                V6
            }
        // 获取枚举值
            let four = IpAddrKind::V4;
            let six = IpAddrKind::V6;
            // 枚举的变体都位于标识符的命名空间下，使用两个冒号::进行分隔

            fn route(ip_kind:IpAddrKind){}

            route(four);
            route(IpAddrKind::V6);

        // 将数据附加到枚举的变体中 
            // 将枚举 加入到 struct 里面
                struct IpAddr{
                    kind: IpAddrKind,
                    address: String,
                }

                let home = IpAddr {
                    kind: IpAddrKind::V4,
                    address: String::from("127.0.0.1"),
                };

                let loopback = IpAddr{
                    kind: IpAddrKind::V6,
                    address: String::from("::1"),
                };

            enum IpAddr_2{
                V4 (String),
                V6 (String),
            }
            // 优点：
                // 不需要额外使用 struct
                // 每个变体可以拥有不同的类型 以及关联的数据量
            
            // 例如：
                enum IpAddr_3 {
                    V4(u8,u8,u8,u8),
                    V6(String),
                }

                let home = IpAddr_3::V4(127, 0, 0, 1);
                let loopback = IpAddr_3::V6(String::from("::1"));

        // 标准库中的IpAddr
            struct Ipv4Addr {

            }

            struct Ipv6Addr {

            }
            // 可以在枚举的变体中 嵌入任意类型的数据，无论是字符串 数值 结构体
            enum IpAddr_4 {
                V4(Ipv4Addr),
                V6(Ipv6Addr)
            }
            let ipv4 = Ipv4Addr {

            };
            let ip = IpAddr_4::V4(ipv4);
            
            // 例子：
                enum Message {
                    Quit,
                    Move {x: i32, y: i32}, // 匿名结构体
                    Write(String),
                    ChangeColor(i32, i32, i32),
                }

                let q = Message::Quit;
                let m = Message::Move { x: 0, y: 10 };
                let w = Message::Write(String::from("Hello"));
                let c = Message::ChangeColor(0, 0, 0);
        // 为枚举定义方法
            // 也使用 impl 关键字
            impl Message {
                fn call(&self) {

                }
            }
            q.call();
    // Option 枚举
        // 标准库中

}

fn match_(){
    // 强大的控制流运算符 - match
        
}

fn gather_vector(){
    // 使用 Vector 存储多个值
        // Vec<T>, 叫做 Vector
        // 由标准库提供
        // 只能存储相同类型的数据
        // 值在内存中连续存放

        // 创建 Vector
            // Vec::new 函数
                let v: Vec<i32> = Vec::new();
            // 使用初始值 创建 Vec<T> , 使用 vec! 宏
                let v = vec![1,2,3];
        // 更新 Vector
            // 向 Vector 添加元素，使用 push 方法
                let mut v = Vec::new();
                v.push(1);
        // 删除 Vector
            // 与任何其他 struct 一样，当 Vector 离开作用域后
                // 它就被清理掉了
                // 它所有的元素也被清理掉了
                fn test(){
                    let v = vec![1, 2, 3];
                } // v 被清掉
        // 读取 Vector 的元素
            // 两种方式可以引用 Vector 里的值：
                // 索引  -> 索引超出 会报错 -- 终止
                let one: &i32 = &v[0]; 
                // get 方法 -> Option<&I::Output> -> 索引超出不会报错，返回 None
                match v.get(2) { 
                    Some(one) => println!("in"),
                    None => println!("not"),
                }
            // 索引 vs get 处理访问越界
                // 索引：panic
                // get：返回 None
        // 所有权和借用规则
            // 不能在同一个作用域内同时拥有可变和不可变引用
                // cannot borrow `v` as mutable, as it is not declared as mutable

                // let v = vec![1, 2, 3, 4, 5];
                // let one = &v[0]; // 不可变的借用
                // v.push(6); // 可变的 cannot borrow as mutable
                // println!("{}", one); // 不可变的
        // 小案例 - for 循环
            let nums = vec![1, 2, 3, 4, 500];
            for num in nums {
                println!("num -> {}", num);
            }
            
            // for num in nums{

            // }


}

mod test;

fn main() {

    // variable(); // 9 变量
    // data_type(); // 10 11  数据类型
    // function(); // 12 函数
    // control(); // 13 14 控制流
    // ownership(); // 15 16 17 所有权
    // reference(); // 18 引用

    // slice(); // 19 切片

    // struct_(); // 20 21 22 struct

    // enum_(); //23 24 枚举  ---- 

    match_(); // 25 控制流运算符

    gather_vector(); // 8-1  33 34 常用的集合 -- 存储在 heap 中

    // test::practise();
    
    // println!("Hello, world!");
}

