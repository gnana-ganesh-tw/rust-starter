fn add_one(num: u32) -> u32 {
    num + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_add_one() {
        assert_eq!(add_one(1), 2);
        assert_eq!(add_one(2), 3);
        assert_eq!(add_one(3), 4);
    }
}
