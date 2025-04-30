use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use indexmap::IndexMap;
use schemars::schema::{InstanceType, SchemaObject, SingleOrVec};

#[derive(Default)]
pub struct Walker {
    objects: Vec<Object>,
}

#[derive(Default)]
struct Object {
    name: String,
    // TODO desc
    filename: Option<String>,
    // name: field
    fields: IndexMap<String, Field>,
}

#[derive(Default)]
struct Field {
    description: Option<String>,
    is_required: bool,
    type_name: String,
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "#![allow(non_camel_case_types)]")?;
        writeln!(f)?;
        for i in &self.objects {
            writeln!(f, "{}", i)?;
        }
        writeln!(f, "#[cfg(test)]")?;
        writeln!(f, "mod tests {{")?;
        writeln!(f, "    use super::*;")?;
        // TODO tests
        // #[test]
        // fn test_add() {
        //     assert_eq!(add(1, 2), 3);
        // }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO desc
        if let Some(filename) = &self.filename {
            writeln!(f, "/// {}", filename)?;
        }
        writeln!(f, "#[derive(Debug, serde::Serialize, serde::Deserialize)]")?;
        writeln!(f, "#[serde(deny_unknown_fields)]")?;
        writeln!(f, "pub struct {} {{", self.name)?;
        for (name, field) in &self.fields {
            debug_assert!(!field.type_name.is_empty());
            if let Some(description) = &field.description {
                writeln!(f, "    /// {}", description)?;
            }
            if field.is_required {
                writeln!(f, "    pub {}: {},", name, field.type_name)?;
            } else {
                writeln!(f, "    pub {}: Option<{}>,", name, field.type_name)?;
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl Walker {
    pub fn run_parse<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();
        let file = File::open(path).unwrap();
        let rdr = BufReader::new(file);
        let schema: SchemaObject = serde_json::from_reader(rdr).unwrap();
        let name = path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        self.parse_object(schema, name, Some(filename));
    }

    fn parse_object(&mut self, schema: SchemaObject, name: String, filename: Option<String>) {
        assert_eq!(Self::get_type(&schema), InstanceType::Object);
        let schema = schema.object.unwrap();
        let mut object = Object {
            name: name.clone(),
            filename,
            ..Default::default()
        };

        for (key, schema) in schema.properties {
            let schema = schema.into_object();
            let mut field = Field::default();

            if let Some(meta) = &schema.metadata {
                if let Some(description) = &meta.description {
                    field.description = Some(description.to_string());
                }
            }

            field.type_name = match Self::get_type(&schema) {
                InstanceType::Null => todo!(),
                InstanceType::Boolean => "bool".into(),
                InstanceType::Object => "/*TODO Object*/".into(),
                InstanceType::Array => {
                    let validator = schema.array.unwrap();
                    let schema = match validator.items.unwrap() {
                        SingleOrVec::Single(v) => *v,
                        SingleOrVec::Vec(_) => unimplemented!(),
                    };
                    let schema = schema.into_object();
                    match Self::get_type(&schema) {
                        InstanceType::Null => todo!(),
                        InstanceType::Boolean => todo!(),
                        InstanceType::Object => {
                            let name = format!("{}__{}", name, key);
                            self.parse_object(schema, name.clone(), None);
                            format!("Vec<{}>", name)
                        }
                        InstanceType::Array => todo!(),
                        InstanceType::Number => todo!(),
                        InstanceType::Integer => todo!(),
                        InstanceType::String => todo!(),
                    }
                }
                InstanceType::Number => todo!(),
                InstanceType::Integer => {
                    let validator = schema.number.unwrap();
                    if validator.minimum.unwrap_or(-1.0) >= 0.0 {
                        "u32".into()
                    } else {
                        "i32".into()
                    }
                }
                InstanceType::String => "String".into(),
            };

            object.fields.insert(key, field);
        } // schema.properties

        for key in &schema.required {
            object.fields.get_mut(key).unwrap().is_required = true;
        }

        self.objects.push(object);
    }

    fn get_type(schema: &SchemaObject) -> InstanceType {
        match schema.instance_type.clone().unwrap() {
            SingleOrVec::Single(v) => *v,
            SingleOrVec::Vec(_) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    #[test]
    fn preserve_order() {
        let mut fields: IndexMap<&str, i32> = IndexMap::new();
        fields.insert("1", 1);
        fields.insert("0", 0);
        assert_eq!(fields["1"], 1);
        assert_eq!(fields["0"], 0);
        let v: Vec<_> = fields.into_iter().collect();
        assert_eq!(v, vec![("1", 1), ("0", 0)]);
    }
}
