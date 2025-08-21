# vis
Procedural macro for setting default scope for struct fields

## Example 1
```rust
use vis::vis;

#[vis(pub)]
pub struct Db {
    count: u8,
    url: String,
    port: u16, 
    password: String,
    username: String,
    tables: Vec<String>,
    indexes: Vec<Index>,
    ...
}
```

## Result 1
```rust
pub struct Db {
    pub count: u8,
    pub url: String,
    pub port: u16,
    pub password: String,
    pub username: String,
    pub tables: Vec<String>,
    pub indexes: Vec<Index>,
}
```

## Example 2
```rust
use vis::vis;

#[vis(pub(crate))]
pub struct Db {
    count: u8,
    pub url: String,
    pub port: u16,
    password: String,
    username: String,
    tables: Vec<String>,
    indexes: Vec<Index>,
    ...
}
```

## Result 2
```rust
pub struct Db {
    pub(crate) count: u8,
    pub url: String,
    pub port: u16,
    pub(crate) password: String,
    pub(crate) username: String,
    pub(crate) tables: Vec<String>,
    pub(crate) indexes: Vec<Index>,
    ...
}
```
