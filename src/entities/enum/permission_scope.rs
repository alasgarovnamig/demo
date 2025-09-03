use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(10))")]
pub enum PermissionScope {
    #[sea_orm(string_value = "own")]
    Own, // Only own resources
    #[sea_orm(string_value = "partner")]
    Partner, // Partner level
    #[sea_orm(string_value = "all")]
    All, // All partners (for main partner)
}

// From ve Into trait'lerini implement etmek için
impl From<PermissionScope> for String {
    fn from(scope: PermissionScope) -> Self {
        match scope {
            PermissionScope::Own => "own".to_string(),
            PermissionScope::Partner => "partner".to_string(),
            PermissionScope::All => "all".to_string(),
        }
    }
}

impl TryFrom<String> for PermissionScope {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "own" => Ok(PermissionScope::Own),
            "partner" => Ok(PermissionScope::Partner),
            "all" => Ok(PermissionScope::All),
            _ => Err(format!("Geçersiz izin kapsamı: {}", value)),
        }
    }
}

// &str için de TryFrom implement edelim
impl TryFrom<&str> for PermissionScope {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}
// #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
// #[sea_orm(rs_type = "i32", db_type = "Integer")]
// pub enum PermissionScope {
//     #[sea_orm(num_value = 0)]
//     Own, // Only own resources
//     #[sea_orm(num_value = 1)]
//     Partner, // Partner level
//     #[sea_orm(num_value = 2)]
//     All, // All partners (for main partner)
// }
