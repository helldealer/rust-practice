fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hello() {
        println!("success");
    }
}