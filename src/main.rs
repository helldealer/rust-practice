fn main() {}

fn add_two_i32(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod test {
    use crate::add_two_i32;

    #[test]
    fn test_add_two_i32() {
        let (a, b) = (1, 2);
        let r = add_two_i32(a, b);
        assert_eq!(r, 3);
    }
}
