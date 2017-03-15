// Problem 4: Find the number of elements in a list.

fn len(v : &[i32]) -> usize {
    return v.len();
}

fn main() {
    assert_eq!(len(&[]), 0);
    assert_eq!(len(&[1]), 1);
    assert_eq!(len(&[1, 2]), 2);
    assert_eq!(len(&[0; 10]), 10);
}
