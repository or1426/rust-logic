use std::vec;
use std::collections::hash_map;
use std::string;
use std::option;

use lval;

pub struct Env{
    stack :  vec::Vec<hash_map::HashMap<string::String, lval::LVal>>,
}

impl Clone for Env{
    fn clone(&self) -> Env {
        Env {
            stack : self.stack.clone()
        }
    }
}

impl Env{
    pub fn new() -> Env{
        Env{
            stack: vec::Vec::new(),
        }
    }

    pub fn push_empty_frame(&mut self){
        self.stack.push(hash_map::HashMap::new());
    }

    pub fn push_frame(&mut self, frame: hash_map::HashMap<string::String, lval::LVal>){
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> option::Option<hash_map::HashMap<string::String, lval::LVal>>{
        self.stack.pop()
    }
    
    pub fn lookup(&self, key: string::String) -> option::Option<lval::LVal>{
        for map in self.stack.iter().rev() {
            match map.get(&key) {
                Some(value) => return Some(value.clone()),
                None => (),
            };
        }

        return None;
    }

    pub fn add_val(&mut self, key: string::String, value: lval::LVal){
        let mut frame = match self.pop_frame() {
            Some(frame) => frame,
            None => hash_map::HashMap::new(),
        };

        frame.insert(key, value);
        self.push_frame(frame);
    }
}
