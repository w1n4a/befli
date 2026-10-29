# Befli

Befli is a minimalistic flag system for rust by w1n4a.

## Why Befli? (features)

### `add` - creates & returns flag:
```rust
    use befli::Core;

    fn main() {
        let x = befli::add("-o"); //name (x is struct)
    }
```
* or
```rust
    use befli::{Core, Syntax};

    fn main() {
        let x = "-o".add(); //name (x is struct)
    }
```

### `get` - returns bool:
```rust
    use befli::Core;

    fn main() {
        let x = befli::add("-o"); //name (x is struct)

        match x.get() { //bool
            true => //code
            false => //code
        }
    }
```

### `collect` - creates an array of flags (struct):
```rust
    use befli::{Core, Multi};

    fn main() {
        let x = ["-o", "-p", "-s"].collect(); //name in array (x is Vec<struct>)
    }
```

### `gets` - returns bool if args contains all flags in the array:
```rust
    use befli::{Core, Multi};

    fn main() {
        match ["-o", "-s"].gets() { //name in array (is bool)
            true => //code
            false => //code
        }
    }
```

### `value` - picks N values after current flag:
```rust
use befli::{Core, Multi};

fn main() {
    let x = befli::add("-p"); //name (x is struct)

    match x.get() {
        true => {
            let i = x.value(1); //i is result (N = 1) (Result<Vec<String>>)
            match i {
                Ok(val) => println!("Hi {}", val[0]), //print
                Err(_) => println!("Error"), //error
            }
        }
        _ => {}
    }
}
```

## License
* Project uses **MIT** license
