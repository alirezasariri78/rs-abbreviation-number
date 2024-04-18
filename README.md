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
    println!("{}", 123.abbreviate_fnumber()); // result: 123
    println!("{}", 123_000.0.abbreviate_fnumber()); //result: 123K

    println!("{}", "1K".unabbreviate_fnumber()); //result: 1000.0
    println!("{}", "1M".unabbreviate_fnumber()); //result: 1000000.0
}

```

Table Of Symbols:

|     Symbol                   |    Base    |
|-----------------------------|--------------|
|         K               |     1_000       |
|         M           |     1_000_000      |
|         G               |     1_000_000_000    |
|      T                 |  1_000_000_000_000 |
|      P                 |    1_000_000_000_000_000  |
|      E                 |      1_000_000_000_000_000_000   |
