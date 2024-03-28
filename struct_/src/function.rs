// 方法语法 https://kaisery.github.io/trpl-zh-cn/ch05-03-method-syntax.html

// 方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文
// 并且它们第一个参数总是 self，它代表调用该方法的结构体实例。

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 这个 impl 块中的所有内容都将与 Rectangle 类型相关联
impl Rectangle {
    // 在 area 的签名中，使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写
    // Self 类型是 impl 块的类型的别名。方法的第一个参数必须有一个名为 self 的Self 类型的参数

    // 方法可以选择获得 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。
    // 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self

    // 通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的
    // 这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 我们可以选择将方法的名称与结构中的一个字段相同
    // 与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。这样的方法被称为 getters
    // Getters 很有用，因为你可以把字段变成私有的，但方法是公共的
    // 这样就可以把对字段的只读访问作为该类型公共 API 的一部分
    fn width(&self) -> bool {
        self.width > 0
    }


    // 带有更多参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }


    // 关联函数
    // 所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 impl 后面命名的类型相关。
    // 我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例

    // 不是方法的关联函数经常被用作返回一个结构体新实例的构造函数。这些函数的名称通常为 new
    fn square(size: u32) -> Self { //关键字 Self 在函数的返回类型中代指在 impl 关键字后出现的类型，在这里是 Rectangle

        Self {
            width: size,
            height: size,
        }
    }

    // 使用结构体名和 :: 语法来调用这个关联函数：比如 let sq = Rectangle::square(3);。
    // 这个函数位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间
}


// 多个 impl 块
// 每个结构体都允许拥有多个 impl 块。例如，示例 5-16 中的代码等同于示例 5-15，但每个方法有其自己的 impl 块。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// 这里没有理由将这些方法分散在多个 impl 块中，不过这是有效的语法。第十章讨论泛型和 trait 时会看到实用的多 impl 块的用例。
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// 当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配
// p1.distance(&p2);
// (&p1).distance(&p2);


