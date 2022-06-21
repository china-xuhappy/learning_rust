mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist(){}
      fn seat_at_table(){
        super::super::eat_at_restaurant(); // 相对路径 ../../
        crate::eat_at_restaurant(); // 绝对路径
      }
  }
  mod serving {
      fn take_order(){}
  }
}

// front_of_house 和 eat_at_restaurant 都是这个文件的根基，私有 也可以访问

pub fn eat_at_restaurant(){
  crate::front_of_house::hosting::add_to_waitlist(); // 绝对
  front_of_house::hosting::add_to_waitlist(); //相对
}

mod back_of_house {

  pub enum Appetizer {
    Soup,
    Salad,
  }
  pub struct Breakfast{
      pub toast: String,
      seasonal_fruit: String,
  }

  impl Breakfast {
    pub  fn summer(toast: &str) -> Breakfast {
        Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
      }
  }
}

// 使用绝对路径 引入
use crate::back_of_house::Breakfast;

// 使用相对路径
// use back_of_house::Breakfast;

pub fn eat_restaurant() {
  let mut meal = Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");

  println!("i'd, like {}", meal.toast);
  // field `seasonal_fruit` of struct `Breakfast` is private
  // meal.seasonal_fruit = String::from("hello");

}