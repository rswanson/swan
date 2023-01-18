pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Response {
    pub message: String,
    pub exit_code: i32,
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
