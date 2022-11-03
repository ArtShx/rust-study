use std::collections::HashMap;

fn myprint<'a, 'b>(x: &'a i32, y: &'b i32) {
    // this func takes 2 references which have differente lifetimes ('a and 'b).
    // these 2 lifetimes must both be at least as long as this function.
    println!("X is \"{}\", Y is \"{}\"", x, y);
}

fn func(){
    let mut x = 2;
    let y = &x;
    // x += 3;  // x was borrowed.
    println!("y: {}", y);
}

// fn failed_borrow<'c>(){
//     // this func takes no arguments but has a lifetime parameter of 'c
//     let x = 0;
//     let y = &x;
//     let z: &'c i32 = &x;  // 'c must live longer than the function, in here z is dropped upon exiting the score
// }

fn main() {
    println!("Hello, world!");
    let (i, j) = (2, -3);
    myprint(&i, &j);

    // does not work
    // failed_borrow();
    func();

    // HashMap tests
    let mut hmap: HashMap<&String, u8> = HashMap::new();
    let key = String::from("Key");
    let key2 = String::from("Key_2");

    hmap.insert(&key, 1 + if true { 2 } else { 0 });
    // hmap.insert(&key, 8);

    // *hmap.get_mut(&key2).unwrap() += 1; // Solution 1, gonna panic if key does not exists
    // Solution 2
    if let Some(x) = hmap.get_mut(&key) {
        *x += 2;
    }

    // does not break if key does not exists
    if let Some(x) = hmap.get_mut(&key2) {
        *x += 2;
    }

    println!("Hmap: {:?}", hmap);
}
