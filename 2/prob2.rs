// Problem 2: Find the second to last element of a list.

fn slast(v : &[i32]) -> i32 {
    return v[v.len()-2];
}

fn main() {
    assert_eq!(slast(&[1,2,3]), 2);
    assert_eq!(slast(&[1,2]), 1);
}
