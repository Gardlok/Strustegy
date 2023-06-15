# Strustegy
Validation library for Rust with emphasis on the Strategy pattern

## Disclaimer
This library is still in early development. Feel free to try it out and give feedback.



## Usage
```rust
use strustegy::Strustegy;

#[derive(Debug, PartialEq)]
enum Error {
    Invalid,
}

#[derive(Debug, PartialEq)]
enum Value {
    Valid,
}

fn main() {
    let mut strustegy = Strustegy::new();

    strustegy.add_validator(|_| Ok(Value::Valid));

    assert_eq!(strustegy.validate(()), Ok(Value::Valid));
}
```






## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)

