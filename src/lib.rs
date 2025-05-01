#![feature(custom_inner_attributes)] // rustfmt::skip

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use schemars::schema::{InstanceType, SchemaObject, SingleOrVec};

// compile tests
#[cfg(test)]
#[path = "../tests/ClsSchemas.rs"]
mod test_cls_schemas;
#[cfg(test)]
#[path = "../tests/DespatchSchemas.rs"]
mod test_despatch_schemas;
#[cfg(test)]
#[path = "../tests/OwnSchemas.rs"]
mod test_own_schemas;

#[derive(Default)]
pub struct Walker {
    objects: Vec<Object>,
}

#[derive(Default)]
struct Object {
    name: String,
    // TODO desc
    path: Option<PathBuf>,
    // <name, field>
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
        writeln!(f, "#![rustfmt::skip]")?;
        writeln!(f)?;
        for i in &self.objects {
            writeln!(f, "{}", i)?;
        }
        // writeln!(f, "#[cfg(test)]")?;
        // writeln!(f, "mod tests {{")?;
        // writeln!(f, "    use super::*;")?;
        // // TODO tests
        // // #[test]
        // // fn test_add() {
        // //     assert_eq!(add(1, 2), 3);
        // // }
        // writeln!(f, "}}")?;
        Ok(())
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO desc
        if let Some(path) = &self.path {
            writeln!(f, "/// {}", path.display())?;
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
    pub fn parse<P: AsRef<Path>>(&mut self, path: P) {
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
        let _ = self.parse_object(schema, name, Some(path.to_path_buf()));
    }

    /// return: type_name
    #[must_use]
    fn parse_any(&mut self, schema: SchemaObject, name: String) -> String {
        match Self::get_type(&schema) {
            Some(InstanceType::Null) => todo!(),
            Some(InstanceType::Boolean) => "bool".into(),
            Some(InstanceType::Object) => self.parse_object(schema, name, None),
            Some(InstanceType::Array) => self.parse_array(schema, name),
            Some(InstanceType::Number) => todo!(),
            Some(InstanceType::Integer) => self.parse_number(schema),
            Some(InstanceType::String) => "String".into(),
            None => todo!(), // TODO reference: Some("#/definitions/int8",
        }
    }

    /// return: type_name
    #[must_use]
    fn parse_object(
        &mut self,
        schema: SchemaObject,
        name: String,
        path: Option<PathBuf>,
    ) -> String {
        debug_assert!(Self::get_type(&schema).unwrap() == InstanceType::Object);
        let validator = match schema.object {
            Some(v) => v,
            None => return "()/*any*/".into(),
        };

        let mut object = Object {
            name: name.clone(),
            path,
            ..Default::default()
        };

        for (key, schema) in validator.properties {
            let schema = schema.into_object();
            let mut field = Field::default();

            if let Some(meta) = &schema.metadata {
                if let Some(description) = &meta.description {
                    field.description = Some(description.to_string());
                }
            }

            let name = format!("{}__{}", name, key);
            field.type_name = self.parse_any(schema, name);

            object.fields.insert(key, field);
        } // schema.properties

        for key in &validator.required {
            object.fields.get_mut(key).unwrap().is_required = true;
        }

        self.objects.push(object);
        name
    }

    /// return: type_name
    #[must_use]
    fn parse_array(&mut self, schema: SchemaObject, name: String) -> String {
        debug_assert!(Self::get_type(&schema).unwrap() == InstanceType::Array);
        let validator = schema.array.unwrap();

        let schema = match validator.items.unwrap() {
            SingleOrVec::Single(v) => *v,
            SingleOrVec::Vec(_) => unimplemented!(),
        };
        let schema = schema.into_object();
        let type_name = self.parse_any(schema, name);
        format!("Vec<{}>", type_name)
    }

    /// return: type_name
    #[must_use]
    fn parse_number(&mut self, schema: SchemaObject) -> String {
        debug_assert!(Self::get_type(&schema).unwrap() == InstanceType::Integer);
        let validator = schema.number.unwrap();

        if validator.minimum.unwrap_or(-1.0) >= 0.0 {
            "u32".into()
        } else {
            "i32".into()
        }
    }

    fn get_type(schema: &SchemaObject) -> Option<InstanceType> {
        match schema.instance_type.clone()? {
            SingleOrVec::Single(v) => Some(*v),
            SingleOrVec::Vec(_) => unimplemented!(),
        }
    }
}
