use std::fmt::Display;

pub trait ParsedCode: Display {
    fn decode(&self) -> Vec<u8>;
}

pub trait CodeFormat {
    fn get_name(&self) -> &'static str;
    fn parse(&self, code: &str) -> Option<Box<ParsedCode>>;
}