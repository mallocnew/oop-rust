// Copyright 2024 JOK Inc. All Rights Reserved.
// Author: easytojoin@163.com (jok)

static mut SINGLETON: Option<Singleton> = Option::None;
static ONCE: std::sync::Once = std::sync::Once::new();
pub struct Singleton {
    data: i32,
}

impl Singleton {
    fn new() -> Self {
        println!("Singleton created");
        Singleton { data: 0 }
    }

    pub fn get_singleton() -> &'static Singleton {
        println!("get_singleton");
        ONCE.call_once(|| unsafe {
            SINGLETON = Some(Singleton::new());
        });
        unsafe { SINGLETON.as_ref().unwrap() }
    }
}
