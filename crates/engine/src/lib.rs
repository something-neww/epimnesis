pub fn plus_100(input: u32) -> u32 {
    input + 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = plus_100(2);
        assert_eq!(result, 102);
    }
}
