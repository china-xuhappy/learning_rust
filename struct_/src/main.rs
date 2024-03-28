// 案例 练习
#[derive(Debug)]
struct Hobby(String, String, String);

struct Test;

#[derive(Debug)]
struct People {
    name: String,
    age: u8,
    hobby: Hobby
}


impl People {
    fn new(name: String, age: u8, hobby: Hobby) -> Self {
        People {
            name,
            age,
            hobby
        }
    }

    fn comparison_age(&self, people: &People) -> String {
        if self.age > people.age {
            self.name.clone()
        }else {
            people.name.clone()
        }
    }
}


fn main() {
    let name = String::from("happy");
    let hobby = Hobby(String::from("test1"), String::from("test2"), String::from("test3"));
    let happy = People::new(name, 18, hobby);

    let hobby2 = Hobby(String::from("test1"), String::from("test2"), String::from("test3"));
    let alice = People::new(String::from("alice"), 20, hobby2);

    println!("age -> {}", happy.comparison_age(&alice));

    dbg!(&happy);

    let hobby = Hobby(String::from("test1"), String::from("test2"), String::from("test3"));

}