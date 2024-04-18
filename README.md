Abbreviate or Unabbreviate numbers in rust


|     Input                   |    Output    |
|-----------------------------|--------------|
|         1_000               |     1K       |
|         1_000_000           |     1M       |
|         1_090               |     1.09K    |
| 123_000_000_000_000_000_000 |     123E     |
| -123_000_000_000_000_000_000 |     -123E     |



|     Input                   |    Output    |
|-----------------------------|--------------|
|         1K               |     1000       |
|         1.09K           |     1090      |
|         1_090               |     1.09K    |
|      123E     |123_000_000_000_000_000_000|


Usage Example :

```rust

fn main() {
    println!("{}", 123.abbreviate_fnumber()); // 123
    println!("{}", 123_000.0.abbreviate_fnumber()); // 123K

    println!("{}", "1K".unabbreviate_fnumber()); // 1000.0
    println!("{}", "1M".unabbreviate_fnumber()); 1000000.0
}

```
