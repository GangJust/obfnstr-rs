# obfnstr-rs

This is a rust string obfuscate lib, Its purpose is to obfuscate as many strings as possible in a fn.

Why as much as possible? Because macro expressions are difficult to handle.

## Currently supported

- fn lit variable

   ```rust
   let name = "Tom"; // -> let name = obfnstr::obfuscate(&[8u8, 117u8, 82u8]);
   ```

- more to be implemented.

## Example

It will try to confuse the strings that appear in the `test` fn as much as possible.

```rust
#[obfnstr]
fn test() {
    println!("Hello, world!");

    let name = "Tom";
    println!("Welcome {}", name);
}
```
