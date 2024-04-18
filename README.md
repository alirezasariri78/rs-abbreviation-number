Abbreviate or Unabbreviate numbers in rust


|     Input                   |    Output    |
|-----------------------------|--------------|
|         1_000               |     1K       |
|         1_000_000           |     1M       |
|         1_090               |     1.09K    |
| 123_000_000_000_000_000_000 |     123E     |
| -123_000_000_000_000_000_000 |     -123E     |


Usage Example :

1. Use As A Extension : 
```

fn main() {
    println!("{}", 123.abbreviate_fnumber());
    println!("{}", 123_000.0.abbreviate_fnumber());

    println!("{}", "1K".unabbreviate_fnumber());
    println!("{}", "1M".unabbreviate_fnumber());
}

```
