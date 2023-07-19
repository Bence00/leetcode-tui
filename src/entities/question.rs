//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use std::cmp::Ordering;

use crate::deserializers::custom_serde::int_from_bool;
use sea_orm::entity::prelude::*;
use serde::{self, Deserialize};

#[derive(Clone, Debug, DeriveEntityModel, Deserialize)]
#[sea_orm(table_name = "Question")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(column_type = "Double", nullable)]
    pub ac_rate: Option<f64>,
    pub difficulty: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub freq_bar: Option<f64>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub frontend_question_id: String,
    #[serde(deserialize_with = "int_from_bool")]
    pub is_favor: Option<i32>,
    #[serde(deserialize_with = "int_from_bool")]
    pub paid_only: Option<i32>,
    pub status: Option<String>,
    pub title: Option<String>,
    pub title_slug: Option<String>,
    #[serde(deserialize_with = "int_from_bool")]
    pub has_solution: Option<i32>,
    #[serde(deserialize_with = "int_from_bool")]
    pub has_video_solution: Option<i32>,
}

impl Eq for Model {}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.frontend_question_id == other.frontend_question_id
    }
}

use std::hash::Hash;

impl Hash for Model {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.frontend_question_id.hash(state);
    }
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self
            .frontend_question_id
            .parse::<i32>()
            .unwrap()
            .partial_cmp(&other.frontend_question_id.parse::<i32>().unwrap())
        {
            Some(core::cmp::Ordering::Equal) => Some(core::cmp::Ordering::Equal),
            ord => return ord,
        }
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frontend_question_id
            .parse::<i32>()
            .unwrap()
            .cmp(&other.frontend_question_id.parse::<i32>().unwrap())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::topic_tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::question_topic_tag::Relation::TopicTag.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::question_topic_tag::Relation::Question.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
