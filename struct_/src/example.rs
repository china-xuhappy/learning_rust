// 结构体示例程序  https://kaisery.github.io/trpl-zh-cn/ch05-02-example-structs.html

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


// 使用元组重构
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// 使用结构体重构：赋予更多意义
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// 通过派生 trait 增加实用功能
    // #[derive(Debug)]
    // println!("rect1 is {:#?}", rect1);

    //     dbg!(&rect1);
    // dbg! 宏接收一个表达式的所有权

