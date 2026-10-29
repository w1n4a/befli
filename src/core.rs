#![allow(dead_code)]

use std::env::args;

pub struct Flag {
    pub name: String,
}

#[derive(Debug)]
pub enum Res {
    None,
}

pub fn add(arg: &str) -> Flag {
    let flag = Flag {
        name: arg.to_string(),
    };

    flag
}

pub trait Core {
    fn get(&self) -> bool;
    fn value(&self, count: usize) -> Result<Vec<String>, Res>;
}

pub trait Multi {
    fn collect(&self) -> Vec<Flag>;
    fn gets(&self) -> bool;
}

pub trait Syntax {
    fn add(&self) -> Flag;
}

impl Syntax for &str {
    fn add(&self) -> Flag {
        let flag = Flag {
            name: self.to_string(),
        };

        flag
    }
}

impl Syntax for String {
    fn add(&self) -> Flag {
        let flag = Flag {
            name: self.to_string(),
        };

        flag
    }
}

impl Multi for [&str] {
    fn collect(&self) -> Vec<Flag> {
        let mut res: Vec<Flag> = Vec::new();

        for i in self {
            res.push(Flag {
                name: i.to_string(),
            });
        }

        res
    }

    fn gets(&self) -> bool {
        let mut res: usize = self.len();
        let args: Vec<String> = args().collect();

        for i in self {
            if args.contains(&i.to_string()) {
                res -= 1;
            }
        }

        match res {
            0 => true,
            _ => false,
        }
    }
}

impl Multi for [String] {
    fn collect(&self) -> Vec<Flag> {
        let mut res: Vec<Flag> = Vec::new();

        for i in self {
            res.push(Flag {
                name: i.to_string(),
            });
        }

        res
    }

    fn gets(&self) -> bool {
        let mut res: usize = self.len();
        let args: Vec<String> = args().collect();

        for i in self {
            if args.contains(&i.to_string()) {
                res -= 1;
            }
        }

        match res {
            0 => true,
            _ => false,
        }
    }
}

impl Multi for Vec<&str> {
    fn collect(&self) -> Vec<Flag> {
        let mut res: Vec<Flag> = Vec::new();

        for i in self {
            res.push(Flag {
                name: i.to_string(),
            });
        }

        res
    }

    fn gets(&self) -> bool {
        let mut res: usize = self.len();
        let args: Vec<String> = args().collect();

        for i in self {
            if args.contains(&i.to_string()) {
                res -= 1;
            }
        }

        match res {
            0 => true,
            _ => false,
        }
    }
}

impl Multi for Vec<String> {
    fn collect(&self) -> Vec<Flag> {
        let mut res: Vec<Flag> = Vec::new();

        for i in self {
            res.push(Flag {
                name: i.to_string(),
            });
        }

        res
    }

    fn gets(&self) -> bool {
        let mut res: usize = self.len();
        let args: Vec<String> = args().collect();

        for i in self {
            if args.contains(&i.to_string()) {
                res -= 1;
            }
        }

        match res {
            0 => true,
            _ => false,
        }
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

    fn value(&self, count: usize) -> Result<Vec<String>, Res> {
        let args: Vec<String> = args().collect();
        let mut res: Vec<String> = Vec::new();

        let mut idx: usize = 0;

        for i in 0..args.len() {
            if args[i] == self.name {
                idx = i
            }
        }

        if idx + count >= args.len() {
            return Err(Res::None);
        }

        match idx {
            0 => Err(Res::None),
            _ => {
                for v in 1..=count {
                    if (idx + v) < args.len() {
                        res.push(args[idx + v].clone());
                    }
                }

                Ok(res)
            }
        }
    }
}
