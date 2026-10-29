# Befli

Befli is a minimalistic flag system for rust by w1n4a.

## Why Befli? (features)

### add - creates & returns flag:
```rust
    use befli::{Core, Create};

    fn main() {
        let x = befli::add("-o"); //name (x is struct)
    }
```

### get - returns bool:
```rust
    use befli::{Core, Create};

    fn main() {
        let x = befli::add("-o"); //name (x is struct)

        match x.get() {
            true => //code
            false => //code
        }
    }
```

### collect - creates an array of flags (struct):
```rust
    use befli::{Core, Create};

    fn main() {
        let x = ["-o", "-p", "-s"].collect(); //name in array (x is Vec<struct>)
    }
```

## License
* Project uses **MIT** license
