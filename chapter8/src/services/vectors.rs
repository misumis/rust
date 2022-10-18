pub fn display_vectors() {
    // Vectors
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    println!("The third element is {}", v.get(2).unwrap());

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}
