fn main() {
    let mut x = 5;
    { // create a scope for the first mutable borrow
        let y = &mut x;
        *y = 10; 
    }
    { //create a scope for the second mutable borrow
        let z = &mut x;
        *z = 12;
    }
    println!("x = {}", x);
}
//or
fn main() {
    let mut x = 5;
    let y = &mut x; // Only one mutable reference
    *y = 10;
    println!("x = {}", x);
}