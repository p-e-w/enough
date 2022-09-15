//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "page")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub time: DateTime,
    pub title: String,
    pub url: String,
    #[sea_orm(column_type = "Text")]
    pub content_markdown: String,
    #[sea_orm(column_type = "Text")]
    pub content_html: String,
    pub is_post: bool,
    pub is_published: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
