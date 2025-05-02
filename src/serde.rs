use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use log::trace;
use rustfmt_wrapper::rustfmt;

use crate::{Schema, SchemaField, SchemaRoot};

#[derive(Debug, Default)]
pub struct Walker {
    definitions: IndexMap<String, String>,
    properties: Vec<Object>,
}

#[derive(Debug, Default)]
struct Object {
    name: String,
    path: Option<PathBuf>,
    // <name, _>
    fields: IndexMap<String, ObjectField>,
}

#[derive(Debug, Default)]
struct ObjectField {
    description: Option<String>,
    type_name: String,
    is_required: bool,
}

impl Walker {
    pub fn parse<P: AsRef<Path>>(&mut self, path: P) {
        trace!("Walker::parse({:?})", path.as_ref());

        let path = path.as_ref();
        let schema = SchemaRoot::try_new(path).unwrap();
        let name = path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        for (name_type, custom_type) in schema.definitions {
            let key = format!("{}__{}", name, name_type);
            match custom_type {
                Schema::Integer => self.definitions.insert(key, "i32".into()),
                Schema::String => self.definitions.insert(key, "String".into()),
                // Schema::Custom(value) => {
                //     self.definitions.insert(key, value);
                // }
                _ => todo!("{:?}", custom_type),
            };
        }

        if let Schema::Object { properties } = schema.schema {
            let _ = self.parse_object(properties, name, Some(path.to_path_buf()));  // TODO description
        } else {
            unreachable!()
        }
    }

    /// return: type_name
    #[must_use]
    fn parse_any(&mut self, schema: Schema, name: String) -> String {
        trace!("Walker::parse_any()");

        match schema {
            Schema::Object { properties } => self.parse_object(properties, name, None),
            Schema::Array { schema } => self.parse_array(*schema, name),
            Schema::Boolean => "bool".into(),
            Schema::Integer => "i32".into(),
            Schema::String => "String".into(),
            Schema::Custom(custom_type) => {
                let key = format!("{}__{}", name.split("__").next().unwrap(), custom_type);
                // inline
                self.definitions[&key].clone()
            }
        }
    }

    /// return: type_name
    #[must_use]
    fn parse_object(
        &mut self,
        properties: Vec<SchemaField>,
        name: String,
        path: Option<PathBuf>,
    ) -> String {
        trace!("Walker::parse_object()");

        let mut fields = IndexMap::new();
        for f in properties {
            let name = format!("{}__{}", name, f.name);
            let value = ObjectField {
                description: f.description,
                type_name: self.parse_any(f.schema, name),
                is_required: f.is_required,
            };
            fields.insert(f.name, value);
        }

        self.properties.push(Object {
            name: name.clone(),
            path,
            fields,
        });
        name
    }

    /// return: type_name
    #[must_use]
    fn parse_array(&mut self, schema: Schema, name: String) -> String {
        trace!("Walker::parse_array()");

        let type_name = self.parse_any(schema, name);
        format!("Vec<{}>", type_name)
    }
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = String::new();
        result = format!("{}{}\n", result, "#![allow(non_camel_case_types)]\n");

        // for (name_type, custom_type) in &self.definitions {
        //     result = format!("{}pub type {} = {};\n", result, name_type, custom_type);
        // }

        for i in &self.properties {
            result = format!("{}{}\n", result, i);
        }

        write!(f, "{}", rustfmt(result).unwrap())
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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
