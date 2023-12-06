fn main(input: &str) -> Option<usize> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    pub fn six() {
        assert_eq!(Some(5), main("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), main("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), main("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(11), main("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
