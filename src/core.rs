#![allow(dead_code)]

use std::env::args;

pub struct Flag {
    pub name: String,
}

pub fn add(arg: &str) -> Flag {
    let flag = Flag {
        name: arg.to_string(),
    };

    flag
}

pub trait Core {
    fn get(&self) -> bool;
}

pub trait Create {
    fn collect(&self) -> Vec<Flag>;
}

impl Create for [&str] {
    fn collect(&self) -> Vec<Flag> {
        let mut res: Vec<Flag> = Vec::new();

        for i in self {
            res.push(Flag {
                name: i.to_string(),
            });
        }

        res
    }
}

impl Core for Flag {
    fn get(&self) -> bool {
        let args: Vec<String> = args().collect();
        if args.contains(&self.name) {
            true
        } else {
            false
        }
    }
}
