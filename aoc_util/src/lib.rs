pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn string_to_lines(text: &str) -> Vec<&str> {
    text.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
