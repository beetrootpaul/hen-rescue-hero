extern crate bevy;

mod def;

pub use crate::def::SomePlugin;

pub use def::unused_two;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn engine_world() -> String {
    "World".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let abc = 123;
        let result = add(2, 2);
        assert_eq!(result, 5);
    }
}
