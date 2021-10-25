fn main() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    v.push(5);

    let first: &i32 = &v1[1];
}