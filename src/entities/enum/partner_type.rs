use std::str::FromStr;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(15))")]
pub enum PartnerType {
    #[sea_orm(string_value = "main")]
    Main,
    #[sea_orm(string_value = "standard")]
    Standard,
    #[sea_orm(string_value = "subsidiary")]
    Subsidiary,
}

// FromStr trait'ini implement etmek için
impl FromStr for PartnerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "main" => Ok(PartnerType::Main),
            "standard" => Ok(PartnerType::Standard),
            "subsidiary" => Ok(PartnerType::Subsidiary),
            _ => Err(format!("Geçersiz partner tipi: {}", s)),
        }
    }
}
// From ve Into trait'lerini implement etmek için
impl From<PartnerType> for String {
    fn from(partner_type: PartnerType) -> Self {
        match partner_type {
            PartnerType::Main => "main".to_string(),
            PartnerType::Standard => "standard".to_string(),
            PartnerType::Subsidiary => "subsidiary".to_string(),
        }
    }
}

impl TryFrom<String> for PartnerType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "main" => Ok(PartnerType::Main),
            "standard" => Ok(PartnerType::Standard),
            "subsidiary" => Ok(PartnerType::Subsidiary),
            _ => Err(format!("Geçersiz partner tipi: {}", value)),
        }
    }
}

// &str için de TryFrom implement edelim (opsiyonel ama kullanışlı)
impl TryFrom<&str> for PartnerType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}
// #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
// #[sea_orm(rs_type = "i32", db_type = "Integer")]
// pub enum PartnerType {
//     #[sea_orm(num_value = 0)]
//     Main,
//     #[sea_orm(num_value = 1)]
//     Standard,
//     #[sea_orm(num_value = 2)]
//     Subsidiary,
// }
