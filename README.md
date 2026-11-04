# Befli

Befli is a minimalistic flag system for rust by w1n4a.
It has two parts: Core and Value. (And error system)

## Core:

### `new` - creates group of flags:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group
    }
```

### `add` - adds flag to group:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag '-s'
    }
```

### `get` - gets current flag for name:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'
        flags.get("-s"); //get flag 's' (returns option)
    }
```

### `get_name` - gets name of flag:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'
        flags.get_name("-s"); //get name flag 's' (returns option)
    }
```

### `get_status` - gets flag status in args:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'
        flags.get_status("-s"); //get status flag 's' (checks flag in args) (returns bool)
    }
```

### `parse` - gets count values after flag:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'

        if flags.get_status("-s") { //check status in args
            let r = flags.parse("-s"); //get values (returns result)

            match r {
                Ok(val) => { /* your code */}
                Err(err) => { /* handle error */}
            }
        }
    }
```

## Value:

### `set_value` - sets count values after flag:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'
        flags.get("-s").set_value(1); //get flag 's' and set value
    }
```

### `clear` - clears flag:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'
        flags.get("-s").clear(); //get flag 's' and clear it
    }
```

### `add_alias` - adds alias for flag:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'
        flags.get("-s").add_alias("--status"); //get flag 's' and add alias
    }
```

### `rem_alias` - removes alias for flag:
```rust
    use befli::Core;

    fn main() {
        let flags = befli::new(); //new group

        flags.add("-s"); //add flag 's'
        flags.get("-s").rem_alias("--status"); //get flag 's' and remove alias
    }
```

## Error System (ErrorTypes) - handling errors:

```rust
    use befli::Core;
    use befli::ErrorsTypes;
    use befli::Value;

    fn main() {
        let mut fl = befli::new();
        fl.add("-p");
        fl.get("-p").set_value(1);

        let i = fl.parse("-p");
        match i {
            Ok(v) => println!("Ok: {:?}", v),
            Err(v) => match v.types {
                ErrorsTypes::Null => println!("Error: Null"), //example
                ErrorsTypes::Few => println!("Error: Few"), //example
                ErrorsTypes::None => println!("Error: None"), //example
            },
        }
    }

```

## License
* Project uses **MIT** license
