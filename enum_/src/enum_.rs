// 枚举定义
// 枚举给予你一个途径去声明某个值是一个集合中的一员


// 任何一个 IP 地址要么是 IPv4 的要么是 IPv6 的，而且不能两者都是
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
// 枚举值
    // 创建 IpAddrKind 两个不同成员的实例：
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 函数接收
    fn route(ip_kind: IpAddrKind) {}
    route(IpAddrKind::V4);


    // 结构体 包含枚举
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 可以使用一种更简洁的方式来表达相同的概念
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));


    // 枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));


    // 标准库是如何定义 IpAddr 
    struct Ipv4Addr {
        // --snip--
    }
    struct Ipv6Addr {
        // --snip--
    }
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    
    // 可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。甚至可以包含另一个枚举！



    // 一个 Message 枚举，其每个成员都存储了不同数量和类型的值
    // 其所有成员都被组合在一起位于 Message 类型下
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // struct 也可以实现
    // 如果我们使用不同的结构体，由于它们都有不同的类型，我们将不能像使用示例 6-2 中定义的 Message 枚举那样，
    // 轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型
    struct QuitMessage; // 类单元结构体
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // 元组结构体
    struct ChangeColorMessage(i32, i32, i32); // 元组结构体


    // enum 也可以在枚举上定义方法
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();



// Option 枚举和其相对于空值的优势
    // Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 Option<T>
    enum Option<T> {
        None,
        Some(T),
    }

    // some_number 的类型是 Option<i32>。some_char 的类型是 Option<char>
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None; // 因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。这里我们告诉 Rust 希望 absent_number 是 Option<i32> 类型的。


    // 编译器不允许像一个肯定有效的值那样使用 Option<T>
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; //  ^ no implementation for `i8 + Option<i8>`


    // 为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 Option<T> 中。接着，当使用这个值时，必须明确的处理值为空的情况

    // 一个值不是 Option<T> 类型，你就 可以 安全的认定它的值不为空


}