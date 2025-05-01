use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use log::trace;
use rustfmt_wrapper::rustfmt;

use crate::{InstanceType, Schema};

#[derive(Default)]
pub struct Walker {
    objects: Vec<Object>,
}

#[derive(Default)]
struct Object {
    name: String,
    path: Option<PathBuf>,
    // <name, _>
    fields: IndexMap<String, Field>,
    // <name, _>
    custom_types: IndexMap<String, CustomType>,
}

#[derive(Default)]
struct Field {
    description: Option<String>,
    type_name: String,
    is_required: bool,
}

struct CustomType {
    type_name: String,
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = String::new();
        result = format!("{}{}\n", result, "#![allow(non_camel_case_types)]\n");
        for i in &self.objects {
            result = format!("{}{}\n", result, i);
        }
        write!(f, "{}", rustfmt(result).unwrap())
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if !self.custom_types.is_empty() {
            for (name, custom_type) in &self.custom_types {
                writeln!(f, "pub type {} = {};", name, custom_type.type_name)?;
            }
            writeln!(f)?;
        }

        if let Some(path) = &self.path {
            writeln!(f, "/// {}", path.display())?;
        }
        writeln!(f, "#[derive(Debug, serde::Serialize, serde::Deserialize)]")?;
        writeln!(f, "#[serde(deny_unknown_fields)]")?;
        writeln!(f, "pub struct {} {{", self.name)?;
        for (name, field) in &self.fields {
            debug_assert!(!field.type_name.is_empty());
            if let Some(description) = &field.description {
                writeln!(f, "/// {}", description)?;
            }
            if field.is_required {
                writeln!(f, "pub {}: {},", name, field.type_name)?;
            } else {
                writeln!(f, "pub {}: Option<{}>,", name, field.type_name)?;
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl Walker {
    pub fn parse<P: AsRef<Path>>(&mut self, path: P) {
        trace!("Walker::parse({:?})", path.as_ref());

        let path = path.as_ref();
        let schema = Schema::try_new(path).unwrap();
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
    fn parse_any(&mut self, schema: Schema, name: String) -> String {
        trace!("Walker::parse_any()");

        match schema.instance_type {
            // Some(InstanceType::Null) => todo!(),
            Some(InstanceType::Boolean) => "bool".into(),
            Some(InstanceType::Object) => self.parse_object(schema, name, None),
            Some(InstanceType::Array) => self.parse_array(schema, name),
            // Some(InstanceType::Number) => todo!(),
            Some(InstanceType::Integer) => self.parse_number(schema),
            Some(InstanceType::String) => "String".into(),
            None => {
                debug_assert!(schema.custom_type.is_some());

                "()/*TODO*/".into()
                // schema.custom_type = Some("#/definitions/int4",
            }
        }
    }

    /// return: type_name
    #[must_use]
    fn parse_object(&mut self, schema: Schema, name: String, path: Option<PathBuf>) -> String {
        trace!("Walker::parse_object()");
        debug_assert_eq!(schema.instance_type, Some(InstanceType::Object));

        let mut object = Object {
            name: name.clone(),
            path,
            ..Default::default()
        };

        if let Some(definitions) = schema.definitions {
            for (key, schema) in definitions {
                let name = format!("{}__{}", name, key);
                let type_name = match schema.instance_type {
                    Some(InstanceType::Integer) => self.parse_number(schema),
                    Some(InstanceType::String) => "String".into(),
                    _ => todo!(),
                };
                let custom_type = CustomType { type_name };
                object.custom_types.insert(name, custom_type);
            }
        }

        if let Some(properties) = schema.properties {
            for (key, schema) in properties {
                let name = format!("{}__{}", name, key);
                let field = Field {
                    description: schema.description.clone(),
                    type_name: self.parse_any(schema, name),
                    is_required: false,
                };
                object.fields.insert(key, field);
            }
        } // schema.properties

        if let Some(required) = schema.required {
            for key in required {
                object.fields.get_mut(&key).unwrap().is_required = true;
            }
        }

        self.objects.push(object);
        name
    }

    /// return: type_name
    #[must_use]
    fn parse_array(&mut self, schema: Schema, name: String) -> String {
        trace!("Walker::parse_array()");
        debug_assert_eq!(schema.instance_type, Some(InstanceType::Array));

        let schema = schema.items.unwrap();
        let type_name = self.parse_any(*schema, name);
        format!("Vec<{}>", type_name)
    }

    /// return: type_name
    #[must_use]
    fn parse_number(&mut self, schema: Schema) -> String {
        trace!("Walker::parse_number()");
        debug_assert_eq!(schema.instance_type, Some(InstanceType::Integer));

        if schema.minimum.unwrap_or(-1) >= 0 {
            "u32".into()
        } else {
            "i32".into()
        }
    }
}
