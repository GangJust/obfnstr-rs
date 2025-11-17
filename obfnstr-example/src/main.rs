use obfnstr_macros::obfnstr;

#[obfnstr]
fn main() {
    test();
    test1("Hello".to_string());
}

#[obfnstr]
fn test() {
    println!("Hello, world!");

    let _a = "Hello";
    let _b = "b1".to_string();
    let _c = 123;
    let _arr = ["hello", "ok"];

    let name = "Tom";
    println!("Welcome {}", name);
    println!("{:?}", _arr);

    let dog = Dog;
    println!("{}", dog.baby_name());
}

#[obfnstr]
fn test1(str: String) -> (String, String) {
    (str, "Test1")
}

trait Animal {
    fn baby_name(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    #[obfnstr]
    fn baby_name(&self) -> String {
        "Spot"
    }
}
