pub fn unused_one() -> i32 {
    123
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_for_game_as_well() {
        assert_eq!("a", "a");
    }
}
