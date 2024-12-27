pub fn encode() -> String {
    "".to_string()
}

pub fn decode(msg: &str) {
    println!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(), "");
    }
}
