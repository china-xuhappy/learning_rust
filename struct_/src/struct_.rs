// 结构体 https://kaisery.github.io/trpl-zh-cn/ch05-00-structs.html
// struct，或者 structure，是一个自定义数据类型，允许你包装和命名多个相关的值，从而形成一个有意义的组合

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 创建结构体
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // 修改值

    // 使用结构体更新语法从其他实例创建实例
        // 使用 user1 中的一个值创建一个新的 User 实例
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };


    // 使用结构体更新语法，我们可以通过更少的代码来达到相同的效果
    // .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // 结构更新语法就像带有 = 的赋值，因为它移动了数据
    // 我们在创建 user2 后不能就再使用 user1 了，因为 user1 的 username 字段中的 String 被移到 user2 中。
    // 如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。active 和 sign_in_count 的类型是实现 Copy trait 的类型




    // 使用没有命名字段的元组结构体来创建不同的类型
        // 元组结构体（tuple structs）
        // 想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了

        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);


    // 没有任何字段的类单元结构体
        // 类单元结构体（unit-like structs） 类似于 ()
        // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
        struct AlwaysEqual;
        let subject = AlwaysEqual;

        // 想我们稍后将为这个类型实现某种行为，使得每个 AlwaysEqual 的实例始终等于任何其它类型的实例，也许是为了获得一个已知的结果以便进行测试。我们无需要任何数据来实现这种行为！



    // 结构体所有权
        // 结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。

        // 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）
        // 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的
        // struct User {
        //     active: bool,
        //     username: &str,
        //     email: &str,
        //     sign_in_count: u64,
        // }

        // let user1 = User {
        //     active: true,
        //     username: "someusername123",
        //     email: "someone@example.com",
        //     sign_in_count: 1,
        // };

}

// 函数构建结构体
fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }

    // 使用字段初始化简写语法
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}