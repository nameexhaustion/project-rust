pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_test() {
        let a = 2;
        let b = 3;
        assert_eq!(add(a, b), 5);
    }
}
