#![warn(rust_2018_idioms)]

pub mod client;
pub mod error;
pub mod models;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
