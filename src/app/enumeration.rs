use sea_orm::prelude::*;
use sea_orm::{ActiveValue, DeriveActiveEnum, EnumIter, IntoActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, DeriveActiveEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "snake_case")]
pub enum Gender
{
    // #[sea_orm(string_value = "01")]
    // #[serde(rename = "01")]
    Male,
    // #[sea_orm(string_value = "02")]
    // #[serde(rename = "02")]
    Female,
}

impl IntoActiveValue<Gender> for Gender
{
    fn into_active_value(self) -> ActiveValue<Gender>
    {
        ActiveValue::Set(self)
    }
}