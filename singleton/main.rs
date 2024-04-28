// Copyright 2024 GOTHAM Inc. All Rights Reserved.
// Author: easytojoin@163.com (jok)

mod singleton;
use singleton::Singleton;

fn main() {
    println!("hungry singleton test");
    let one = Singleton::get_singleton();
    println!("{:p}", one);
    let two = Singleton::get_singleton();
    println!("{:p}", two);
    let three = Singleton::get_singleton();
    println!("{:p}", three);
}
