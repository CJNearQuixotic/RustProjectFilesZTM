fn main() {}

fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_function_adds_two_numbers() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn add_function_adds_two_negative_numbers() {
        let result = add(-1, 4);
        assert_eq!(result, 3);
    }
}