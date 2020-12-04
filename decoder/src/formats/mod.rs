pub trait ShellcodeFormat {
    fn get_name(&self) -> &'static str;
    fn decode(&self, code: &str) -> Option<Vec<u8>>;
}

pub mod unicode;
pub mod ascii;