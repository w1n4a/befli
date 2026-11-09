#![allow(dead_code)]

use std::env::args;

#[derive(Debug, Clone)]
pub struct Flag {
    pub name: String,
    pub value: usize,
    pub alias: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub flags: Vec<Flag>,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorsTypes {
    Null,
    Few,
    None,
}

#[derive(Debug, Clone)]
pub struct Errors {
    pub types: ErrorsTypes,
    pub value: String,
}

pub fn new() -> Flags {
    let flag = Flags {
        flags: Vec::new(),
        args: args().collect(),
    };

    flag
}

pub trait Core {
    fn add(&mut self, fl: &str);
    fn get(&mut self, fl: &str) -> Option<&mut Flag>;
    fn get_name(&self, fl: &str) -> Option<String>;
    fn get_status(&self, fl: &str) -> bool;
    fn get_pos(&self, fl: &str) -> Option<usize>;
    fn parse(&self, fl: &str) -> Result<Vec<String>, Errors>;
}

pub trait Value {
    fn set_value(&mut self, val: usize) -> Option<&mut Flag>;
    fn clear(&mut self) -> Option<&mut Flag>;
    fn add_alias(&mut self, val: &str) -> Option<&mut Flag>;
    fn rem_alias(&mut self, val: &str) -> Option<&mut Flag>;
}

impl Core for Flags {
    fn add(&mut self, fl: &str) {
        let mut status = true;

        for i in &self.flags {
            if i.name == fl {
                status = false;
            }
        }

        match status {
            x if x => {
                let flag = Flag {
                    name: fl.to_string(),
                    value: 0,
                    alias: Vec::new(),
                };
                self.flags.push(flag);
            }
            _ => {}
        }
    }
    fn get(&mut self, fl: &str) -> Option<&mut Flag> {
        for i in &mut self.flags {
            if i.name == fl {
                return Some(i);
            }
        }
        None
    }
    fn get_name(&self, fl: &str) -> Option<String> {
        for i in &self.flags {
            if i.name == fl {
                return Some(i.name.clone());
            }
        }

        None
    }
    fn get_status(&self, fl: &str) -> bool {
        let mut status = false;

        for v in &self.flags {
            if v.name == fl || v.alias.contains(&fl.to_string()) {
                for i in &self.args {
                    if *i == v.name || v.alias.contains(i) {
                        status = true
                    }
                }
            }
        }

        status
    }
    fn parse(&self, fl: &str) -> Result<Vec<String>, Errors> {
        let mut pos: usize = 0;
        let mut val: usize = 0;
        let mut flags: Vec<String> = Vec::new();

        let mut flag_name = "";
        let mut flag_val: usize = 0;

        for i in &self.flags {
            flags.push(i.name.clone());

            for v in &i.alias {
                flags.push(v.clone());
            }

            if i.name == fl || i.alias.contains(&fl.to_string()) {
                flag_name = &i.name;
                flag_val = i.value.clone();

                for v in 0..self.args.len() {
                    if self.args[v] == i.name || i.alias.contains(&self.args[v].to_string()) {
                        pos = v;
                        val = i.value;
                    }
                }

                if i.value == 0 {
                    return Err(Errors {
                        types: ErrorsTypes::Null,
                        value: format!("Flag '{}' has a null value", flag_name).to_string(),
                    });
                }

                if pos + i.value >= self.args.len() {
                    return Err(Errors {
                        types: ErrorsTypes::Few,
                        value: format!("Too few args: expected {}", flag_val).to_string(),
                    });
                }
            }
        }

        match pos {
            0 => {
                return Err(Errors {
                    types: ErrorsTypes::None,
                    value: format!("Flag '{}' not in args", flag_name).to_string(),
                })
            }
            _ => {
                let mut res: Vec<String> = Vec::new();
                for v in 1..=val {
                    let f = &self.args[pos + v];
                    if flags.contains(f) == true {
                        return Err(Errors {
                            types: ErrorsTypes::Few,
                            value: format!("Too few args: expected {}", flag_val).to_string(),
                        });
                    } else {
                        res.push(f.clone());
                    }
                }
                Ok(res)
            }
        }
    }
    fn get_pos(&self, fl: &str) -> Option<usize> {
        let mut pos: usize = 0;
        for i in 0..self.args.len() {
            if &self.args[i] == fl {
                pos = i;
            }
        }

        match pos {
            0 => None,
            _ => Some(pos),
        }
    }
}

impl Value for Option<&mut Flag> {
    fn set_value(&mut self, val: usize) -> Option<&mut Flag> {
        match self {
            Some(v) => {
                v.value = val;
                Some(v)
            }
            None => None,
        }
    }

    fn clear(&mut self) -> Option<&mut Flag> {
        match self {
            Some(v) => {
                v.value = 0;
                for _ in 0..v.alias.len() {
                    v.alias.remove(0);
                }
                Some(v)
            }
            None => None,
        }
    }

    fn add_alias(&mut self, val: &str) -> Option<&mut Flag> {
        match self {
            Some(v) => {
                v.alias.push(val.to_string());
                Some(v)
            }
            None => None,
        }
    }

    fn rem_alias(&mut self, val: &str) -> Option<&mut Flag> {
        match self {
            Some(v) => {
                for i in 0..v.alias.len() {
                    if v.alias[i] == val {
                        v.alias.remove(i);
                    }
                }
                Some(v)
            }
            None => None,
        }
    }
}
