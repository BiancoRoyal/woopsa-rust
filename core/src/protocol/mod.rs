pub mod constant;
pub mod container;
pub mod element;
pub mod error_type;
pub mod link;
pub mod method;
pub mod method_argument_info;
pub mod object;
pub mod property;
pub mod url;
pub mod value;
pub mod value_type;
pub mod verb;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
