// Problem 5: Reverse a list.

fn reverse(v: &[i32]) -> Vec<i32> {
    let mut r : Vec<i32> = Vec::new();
    for x in v {
        r.insert(0, *x);
    }
    return r;
}

fn main() {
    assert_eq!(reverse(&[1,2,3]), vec![3,2,1]);
    assert_eq!(reverse(&[]), vec![]);
}
