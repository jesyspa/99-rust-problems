// Problem 1: Find the last element of a list.

fn last(v : &[i32]) -> i32 {
    return v[v.len()-1];
}

fn main() {
    assert_eq!(last(&[1, 2, 3]), 3);
    assert_eq!(last(&[1]), 1);
}
