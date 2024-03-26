// 所有权 练习
fn main() {
    let s = String::from("hello");
    println!("s value {}", s);

    let mut s2 = s; // s的栈信息 copy 给s2
    // println!("s value {}", s); // s 移动到 s2 s不再可用
    s2.push_str("string");
    println!("s value {}", s2);  // hellostring


    let mut n1 = 55;
    println!("n1 value {}", n1);

    let mut n2 = n1; // n1 实现了copy trait，而且值都在栈上，不会出现移动
    println!("n1 value {}", n1); // 55

    n2 = 33; // 变更n2 不会 改变n1
    n1 = 44;
    println!("n1 value {}", n1); // 44
    println!("n2 value {}", n2); // 33

    
    // 函数所有权
    let str = String::from("value");
    let num1 = 333;
    let mut str2 = test_string(str, num1); // str 移动到 函数内
    // println!("str value {}", str); //str 不再可用
    println!("num1 value {}", num1); //num1 实现了copy，后续仍然可用

    str2.push_str("string");
    println!("str2 value {}", str2); //str2 接受函数的返回值，获得所有权

}

fn test_string(string: String, num: u32) -> String { // 创建了一个函数，参数string 需要 String的所有权, num 需要 u32类型
    println!("test_string value {}  num {}", string, num);

    String::from("value") // 返回了一个 String， 移动给接收人
} // 离开作用域  销毁 string变量