

//use super::*;

use super::System;

pub struct MoveSys {
    pub v: i32,
}

impl MoveSys {
    pub fn new() -> MoveSys {
        MoveSys {
            v: 1024,
        }
    }
}

impl System for MoveSys {
    fn update(&self) {
        todo!()
    }
}
