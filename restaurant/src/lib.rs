#![allow(unused)]

mod front_of_house;

use crate::front_of_house::{hosting, serving as serves};
use std::{collections::HashMap, fs::File};


pub fn eat_at_restaurant() {
    // hosting::add_to_waitlist();
    // hosting::add_to_waitlist();
    // hosting::add_to_waitlist();

    // hosting::seat_at_table();

    // serves::serve_order();

    // let mut v: Vec<i32> = Vec::new();
    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();
    // let mut s1 = String::from(s);
    // s1.push_str("bar");


    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let third: &i32 = &v[2];
    // println!("Third: {}", third);

    // match v.get(2) {
    //     Some(third) => println!("WOO{}", third),
    //     None => println!("NOO"),
    // }

    // let first = &v[0];

    // v.push(9);

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yello"), 50);

    let f = File::open("Hello.txt");

}
