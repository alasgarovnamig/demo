use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(10))")]
pub enum UserType {
    #[sea_orm(string_value = "system")]
    System, // Application user
    #[sea_orm(string_value = "portal")]
    Portal, // Portal user for operations/finance
    #[sea_orm(string_value = "admin")]
    Admin, // Partner admin
    #[sea_orm(string_value = "operator")]
    Operator, // Regular operator
}

use std::fmt;

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_value = match self {
            UserType::System => "system",
            UserType::Portal => "portal",
            UserType::Admin => "admin",
            UserType::Operator => "operator",
        };
        write!(f, "{}", str_value)
    }
}
// From ve Into trait'lerini implement etmek için
impl From<UserType> for String {
    fn from(user_type: UserType) -> Self {
        match user_type {
            UserType::System => "system".to_string(),
            UserType::Portal => "portal".to_string(),
            UserType::Admin => "admin".to_string(),
            UserType::Operator => "operator".to_string(),
        }
    }
}

impl TryFrom<String> for UserType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "system" => Ok(UserType::System),
            "portal" => Ok(UserType::Portal),
            "admin" => Ok(UserType::Admin),
            "operator" => Ok(UserType::Operator),
            _ => Err(format!("Geçersiz kullanıcı tipi: {}", value)),
        }
    }
}

// &str için de TryFrom implement edelim
impl TryFrom<&str> for UserType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}
//
// #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
// #[sea_orm(rs_type = "i32", db_type = "Integer")]
// pub enum UserType {
//     #[sea_orm(num_value = 0)]
//     System, // Application user
//     #[sea_orm(num_value = 2)]
//     Portal, // Portal user for operations/finance
//     #[sea_orm(num_value = 3)]
//     Admin, // Partner admin
//     #[sea_orm(num_value = 4)]
//     Operator, // Regular operator
// }
