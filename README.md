# json-2-rust-types-generator
Rust crate that can generate Rust types from a JSON schema


## Usage

To use the json-schema-to-rust crate, add it as a dependency in your Cargo.toml file:

```
[dependencies]
json-schema-to-rust = "0.1"
```

Then, you can use the generate_types function to generate Rust types from a JSON schema.

The json-schema-to-rust crate supports a subset of the JSON schema specification, including $ref, allOf, oneOf, anyOf, and type.

### License

This crate is licensed under the MIT License. See the LICENSE file for details.
