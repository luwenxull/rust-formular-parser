pub fn some<T>(list: &Vec<T>, predicate: impl Fn(&T) -> bool) -> bool {
    for item in list {
        if predicate(item) {
            return true;
        }
    }
    return false;
}
