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


- http://play.rust-lang.org/
- https://doc.rust-lang.org/stable/std/
- https://crates.io/
- https://users.rust-lang.org