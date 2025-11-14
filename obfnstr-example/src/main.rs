use obfnstr_macros::obfnstr;

fn main() {
    test()
}

#[obfnstr]
fn test() {
    println!("Hello, world!");

    let name = "Tom";
    println!("Welcome {}", name);
}
