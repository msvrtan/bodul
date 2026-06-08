# Coding Guidelines

## Rust style

### Always return named bindings

Functions must bind the return value to a named variable before returning. Do not return expressions or call chains directly.

```rust
// bad
fn load(&self) -> Result<Config, Error> {
    self.store.find().ok_or(Error::NotFound)
}

// good
fn load(&self) -> Result<Config, Error> {
    let config = self.store.find().ok_or(Error::NotFound)?;
    Ok(config)
}
```

### Write public methods as intention, not implementation

Public methods must read as a sequence of named business steps. Extract all mechanics into private methods named after what they do.

```rust
// bad
pub async fn handle(&self, command: DoThing) -> Result<Output, Error> {
    let row = self.db.query("SELECT ...").map_err(Error::Db)?;
    let mut results = Vec::new();
    for item in row { ... }
    Ok(Output { results })
}

// good
pub async fn handle(&self, command: DoThing) -> Result<Output, Error> {
    let thing = self.load_thing(&command.id)?;
    let processed = self.process(thing)?;
    let output = Self::build_output(processed);
    Ok(output)
}
```
