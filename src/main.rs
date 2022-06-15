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
            
}       

fn main() {
    
    // variable(); // 9 变量
    // data_type(); // 10 11  数据类型
    // function(); // 12 函数
    // control(); // 13 14 控制流

    ownership(); // 15 16 17 所有权
    
    // println!("Hello, world!");
}
