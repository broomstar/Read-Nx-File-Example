use nx;
use nx::GenericNode;
use std::ops::Deref;

pub struct MyNode<'a>(pub nx::Node<'a>);

impl<'a> Deref for MyNode<'a> {
    type Target = nx::Node<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> MyNode<'a> {
    pub fn get_i16(&self) -> i16 {
        match self.0.dtype() {
            nx::Type::Integer => if let Some(int) = self.0.integer() {
                return int as i16;
            },
            nx::Type::String => if let Some(strr) = self.0.string() {
                return strr.parse().unwrap();
            },
            _ => return 0,
        }
        return 0;
    }

    pub fn get_i32(&self) -> i32 {
        match self.0.dtype() {
            nx::Type::Integer => if let Some(int) = self.0.integer() {
                return int as i32;
            },
            nx::Type::String => if let Some(strr) = self.0.string() {
                return strr.parse().unwrap();
            },
            _ => return 0,
        }
        return 0;
    }
}
