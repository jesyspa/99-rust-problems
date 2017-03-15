// Problem 3: Find the k-th element of a list.

fn kth(v : &[i32], k : usize) -> i32 {
    return v[k];
}

fn main() {
    assert_eq!(kth(&[1, 2, 3], 0), 1);
    assert_eq!(kth(&[1, 2, 3], 1), 2);
    assert_eq!(kth(&[1], 0), 1);
}
