## System Command

### `std::process::Command` 

https://doc.rust-lang.org/stable/std/process/struct.Command.html

```
use std::process::Command;

let output = Command::new("sh")
                     .arg("-c")
                     .arg("echo hello")
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
let hello = output.stdout;
```
## Bitwise

```rust
fn main() {
    let v = 1;
    for variable in -10..10 {
        println!("{}&{}=>{}", variable, v, variable & v);
    }
    for variable in -10..10 {
        println!("{}&{}=>{}", variable, v, variable & v);
    }
    for variable in -10..10 {

        println!("{}|{}=>{}", variable, v, variable | v);
    }
    for variable in -10..10 {

        println!("{}>>{}=>{}", variable, v, variable >> v);
    }
    for variable in -10..10 {

        println!("{}<<{}=>{}", variable, v, variable << v);
    }
}

```


- http://man7.org/linux/man-pages/man3/strftime.3.html
- http://play.rust-lang.org/
- https://crates.io/
- https://doc.rust-lang.org/stable/std/
- https://doc.rust-lang.org/time
- https://github.com/hyperium/hyper
- https://github.com/rust-lang-deprecated/time
- https://users.rust-lang.org
- http://doc.rust-lang.org/rustc-serialize
http://jgallagher.github.io/rusqlite/rusqlite/index.html

    strftime - format date and time
    
    
fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T

let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);