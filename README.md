# List Apps

A simple cross-platform crate that lists all the apps installed on a system.
Windows and MacOS are supported.

## Usage

Single entrypoint is `installed::list()` which returns an iterator of `App`s.
Each `App` has standardized accessor functions to get metadata.

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let apps = installed::list()?;
    for app in apps {
        // metadata accessor fns, these are only evaluated when used
        let name = app.name();
        let version = app.version();
        let publisher = app.publisher();
        println!("{name} v{version} by {publisher}");
    }
    Ok(())
}
```

---

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>