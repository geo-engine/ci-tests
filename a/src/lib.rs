pub fn foo() {
    println!("foo");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_foo() {
        foo();
    }
}
