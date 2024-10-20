// testing public function
pub fn say_hello(name: String) -> String {
    format!("Hello {}!", name)
}

// testing private function
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        assert_eq!(say_hello(String::from("world")), "Hello world!");
    }

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
