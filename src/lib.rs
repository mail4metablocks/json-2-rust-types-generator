extern crate serde_derive;
extern crate serde_json;

use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Type {
    #[serde(rename = "type")]
    rust_type: String,
    properties: Option<Value>,
    items: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct Schema {
    #[serde(rename = "$ref")]
    reference: Option<String>,
    all_of: Option<Vec<Type>>,
    one_of: Option<Vec<Type>>,
    any_of: Option<Vec<Type>>,
    type_: Option<Type>,
}

pub fn generate_types(schema: &str) -> Vec<String> {
    let mut generated_types = Vec::new();

    let schema: Schema = serde_json::from_str(schema).unwrap();

    if let Some(ref_path) = schema.reference {
        let reference_type = ref_path.split("/").last().unwrap();
        generated_types.push(reference_type.to_pascal_case());
    } else if let Some(all_of) = schema.all_of {
        for t in all_of {
            let type_name = t.rust_type.to_pascal_case();
            generated_types.push(type_name);
        }
    } else if let Some(one_of) = schema.one_of {
        for t in one_of {
            let type_name = t.rust_type.to_pascal_case();
            generated_types.push(type_name);
        }
    } else if let Some(any_of) = schema.any_of {
        for t in any_of {
            let type_name = t.rust_type.to_pascal_case();
            generated_types.push(type_name);
        }
    } else if let Some(t) = schema.type_ {
        let type_name = t.rust_type.to_pascal_case();
        generated_types.push(type_name);
    }

    generated_types
}

trait ToPascalCase {
    fn to_pascal_case(&self) -> String;
}

impl ToPascalCase for str {
    fn to_pascal_case(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
