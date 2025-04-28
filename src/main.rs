#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::BufReader;

use schemars::schema::{InstanceType, SchemaObject, SingleOrVec};

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct achievement_cls {
    pub achievement_cls: Vec<achievement_cls__inner>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct achievement_cls__inner {
    /// Идентификатор
    pub id: u32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

fn get_type(schema: &SchemaObject) -> InstanceType {
    match schema.instance_type.clone().unwrap() {
        SingleOrVec::Single(v) => return *v,
        SingleOrVec::Vec(_) => todo!(),
    }
}

fn parse(name: String, schema: SchemaObject) {
    match get_type(&schema) {
        InstanceType::Null => todo!(),
        InstanceType::Boolean => todo!(),
        InstanceType::Object => {
            println!(
                r##"#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]"##
            );
            println!(r##"pub struct {name} {{"##);
            let schema = schema.object.unwrap();
            for (key, schema) in schema.properties {
                let schema = schema.into_object();
                if let Some(meta) = &schema.metadata {
                    if let Some(desc) = &meta.description {
                        println!(r##"    /// {desc}"##);
                    }
                } 
                match get_type(&schema) {
                    InstanceType::Null => todo!(),
                    InstanceType::Boolean => {
                        println!(r##"    pub {key}: bool,"##);
                    }
                    InstanceType::Object => todo!(),
                    InstanceType::Array => {
                        // let validator = schema.array.unwrap();
                        // TODO case if i32?
                        // TODO save into array
                        let name = key.clone() + "__inner";
                        println!(r##"    pub {key}: Vec<{name}>,"##);
                        println!(r##"}}"##);
                        println!(r##""##);
                        parse(name, schema);
                        println!(r##"}}"##);
                        println!(r##""##);
                    }
                    InstanceType::Number => todo!(),
                    InstanceType::String => {
                        println!(r##"    pub {key}: String,"##);
                    }
                    InstanceType::Integer => {
                        let validator = schema.number.unwrap();
                        if validator.minimum.unwrap_or(-1.0) >= 0.0 {
                            println!(r##"    pub {key}: u32,"##);
                        } else {
                            println!(r##"    pub {key}: i32,"##);
                        }
                    }
                }
            }
        }
        InstanceType::Array => {
            let schema = schema.array.unwrap();
            match schema.items.unwrap() {
                SingleOrVec::Single(v) => parse(name, (*v).into_object()),
                SingleOrVec::Vec(_) => todo!(),
            }
        }
        InstanceType::Number => todo!(),
        InstanceType::String => todo!(),
        InstanceType::Integer => todo!(),
    }
}

fn main() {
    println!(r##"#![allow(non_camel_case_types)]"##);
    println!();

    let file = File::open("achievement_cls.json").unwrap();
    let rdr = BufReader::new(file);
    let schema: SchemaObject = serde_json::from_reader(rdr).unwrap();
    parse("achievement_cls".into(), schema);
}
