use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(10))")]
pub enum HttpMethod {
    #[sea_orm(string_value = "GET")]
    Get,
    #[sea_orm(string_value = "POST")]
    Post,
    #[sea_orm(string_value = "PUT")]
    Put,
    #[sea_orm(string_value = "DELETE")]
    Delete,
    #[sea_orm(string_value = "PATCH")]
    Patch,
}

// Ayrıca From ve Into trait'lerini implement etmek faydalı olabilir
impl From<HttpMethod> for String {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Put => "PUT".to_string(),
            HttpMethod::Delete => "DELETE".to_string(),
            HttpMethod::Patch => "PATCH".to_string(),
        }
    }
}

impl TryFrom<String> for HttpMethod {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(format!("Geçersiz HTTP method: {}", value)),
        }
    }
}
// #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
// #[sea_orm(rs_type = "i32", db_type = "Integer")]
// pub enum HttpMethod {
//     #[sea_orm(num_value = 0)]
//     Get,
//     #[sea_orm(num_value = 1)]
//     Post,
//     #[sea_orm(num_value = 2)]
//     Put,
//     #[sea_orm(num_value = 3)]
//     Delete,
//     #[sea_orm(num_value = 4)]
//     Patch,
// }