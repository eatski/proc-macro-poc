use std::collections::HashMap;

use nobori::TesTes;

pub trait ToSchema  {
    fn to_schema() -> Schema;
}

#[derive(Debug)]
pub enum Schema {
    Struct(HashMap<String,Schema>),Enum(Vec<(String,EnumType)>),String
}

#[derive(Debug)]
pub enum EnumType {
    End,
    HasNamedContent(HashMap<String,Schema>),
    HasUnNamedContent(Vec<Schema>)
}

trait SchemaNew<T> {
    fn new() -> Self;
}

impl SchemaNew<String> for Schema {
    fn new() -> Self {
        Schema::String
    }
}

impl<T: ToSchema> SchemaNew<T> for Schema {
    fn new() -> Self {
        T::to_schema()
    }
}

#[derive(TesTes)]
pub struct Sample {
    pub hoge: Hoge,
    pub fuga: Fuga
}

#[derive(TesTes)]
pub struct Hoge {
    pub name: String
}

pub enum Fuga {
    Fu,Ga
}

impl ToSchema for Fuga {
    fn to_schema() -> Schema {
        Schema::Enum(
            vec![
                ("Fu".to_string(),EnumType::End),
                ("Ga".to_string(),EnumType::End)
            ]
        )
    }
}

fn main() {
    let schema = Sample::to_schema();
    println!("{:?}",schema);
}