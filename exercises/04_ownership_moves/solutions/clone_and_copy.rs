fn main() {
    let x = 42;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let a = true;
    let b = a;
    println!("a = {}, b = {}", a, b);

    let s1 = String::from("owned");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
}
