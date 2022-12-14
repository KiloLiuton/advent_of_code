pub fn get_nth_word(s: &str, nth: usize) -> Option<&str> {
    s.split(' ').skip(nth - 1).next()
}
