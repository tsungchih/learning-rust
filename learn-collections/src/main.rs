fn main() {
    let v = vec![1, 2, 3];
    println!("Hello, world! {:?}", v);
    let mut v1: Vec<String> = Vec::new();
    v1.push(String::from("Hello,"));
    v1.push(String::from("World!"));
    println!("{}", v1.capacity());
    for element in v.iter() {
        println!("element: {}", element);
        println!("element * 2 = {}", element * 2);
    }
    for idx in 0..v1.capacity() {
        match v1.get(idx) {
            Some(s) => println!("element: {}", s),
            None => println!("None!!!"),
        };
    }
    for element in &mut v1 {
        println!("The element is: {}.", element);
    }
}
