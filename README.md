## **Abbreviate or Unabbreviate numbers in rust**

## For Numbers Less Then 1
|     Input                   |    Output    |
|-----------------------------|--------------|
|         0.1           |     100m       |
|         0.01           |     10m       |
|         0.001           |     1m       |
|         0.000_000_001           |     1n       |
|         0.000_000_000_000_000_000_000_000_000_1           |     100q       |


## For Numbers Greater Then 1
|     Input                   |    Output    |
|-----------------------------|--------------|
|         1_000               |     1K       |
|         1_000_000           |     1M       |
|         1_090               |     1.09K    |
|         001_090               |     1.09K    |
| 123_000_000_000_000_000_000 |     123E     |
| -123_000_000_000_000_000_000 |     -123E     |
| 1_000_000_000_000_000_000_000_000_000_000 |     1Q     |




|     Input                   |    Output    |
|-----------------------------|--------------|
|         1K               |     1000       |
|         1.09K           |     1090      |
|         1_090               |     1.09K    |
|      123E     |123_000_000_000_000_000_000|
|      1Q     |1_000_000_000_000_000_000_000_000_000_000|


## Usage Example :

```rust

fn main() {

    // numbers greater then 1:
    println!("{}", 123.abbreviate_number(&Default::default())); // result: 123
    println!("{}", 123_000.0.abbreviate_number(&Default::default())); //result: 123K

    println!("{}", "1K".unabbreviate_number(&Default::default())); //result: 1000.0
    println!("{}", "1M".unabbreviate_number(&Default::default())); //result: 1000000.0

    // numbers less then 1:
    println!("{}", (0.1).abbreviate_number(&Default::default())); // result: 100m
    println!("{}",(0.000_000_001).abbreviate_number(&Default::default())); //result: 1n

    println!("{}", "111z".unabbreviate_number(&Default::default())); // result : 0.000000000000000000111
    println!("{}", "100n".unabbreviate_number(&Default::default())); // result : 0.0000001

}

```

## Also For Big Int Numbers: 
```rust
fn main() {
    let big_num: i128 = 999_509_999_999_999_000_123_123_000_000_123;
    println!("{}", big_num.abbreviate_number(&Default::default())); // result : 999.5Q
}

```

## Customize Format:
```rust
fn main(){
 assert_eq!(
            "100-q",
            (0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&AbbreviationOptions {
                padding: 0,
                separator: "-".to_string()
            })
        );

 assert_eq!(
            "-100   q",
            (-0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&AbbreviationOptions {
                padding: 3,
                ..Default::default()
            })
        );
}
```

## Note:
Symbols Are Case-Sensitive And
Program Will Automaticly Remove Leading Zero (001 Will Become 1 and etc)



## Table Of Metric Prefix:

|     Symbol                   |    Base    |
|-----------------------------|--------------|
|      q                |      $`10^ {-30}`$   |
|     r                 |      $`10^ {-27}`$   |
|     y                 |      $`10^ {-24}`$  |
|     z                 |      $`10^ {-21}`$   |
|     a                 |      $`10^ {-18}`$   |
|      f                 |      $`10^ {-15}`$  |
|             p          |      $`10^ {-12}`$ |
|         n               |     $`10^ {-9}`$    |
|         μ              |      $`10^ {-6}`$      |
|         m               |     $`10^ {-3}`$       |
|                     |      $`10^{0}`$   |
|         K               |     $`10^{3}`$       |
|         M               |      $`10^ {6}`$      |
|         G               |     $`10^ {9}`$    |
|      T                 |      $`10^ {12}`$ |
|      P                 |      $`10^ {15}`$  |
|      E                 |      $`10^ {18}`$   |
|      Z                 |      $`10^ {21}`$   |
|      Y                 |      $`10^ {24}`$  |
|      R                 |      $`10^ {27}`$   |
|      Q                 |       $`10^{30}`$    |



 ## TODO :
```[tasklist]
- [ ] Optimize Code
- [ ] Create Custom Symbols 
- [x] Custom separator pass by option
- [ ] Create Unit Conversion 
```


