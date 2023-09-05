pub fn push_front(mut s: String, prefix: &str) -> String {
    s.insert_str(0, prefix);
    s
}
