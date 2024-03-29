use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::time::Instant;

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let start = Instant::now();

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    let duration = start.elapsed();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}, untyped cost {:?}", v["name"], v["phones"][0], { duration });

    let start = Instant::now();

    // serialize
    let sjs = serde_json::to_string(&v)?;

    let duration = start.elapsed();
    println!("Serde Value to string: {}, untyped cost {:?}", sjs, duration);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let start = Instant::now();

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    let duration = start.elapsed();

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}, typed cost {:?}", p.name, p.phones[0], duration);

    let start = Instant::now();

    // serialize
    let sjs = serde_json::to_string(&p)?;

    let duration = start.elapsed();
    println!("Struct to string: {}, typed cost {:?}", sjs, duration);

    Ok(())
}

fn main() {
    let _ = untyped_example();
    let _ = typed_example();
}
