// // src/services/opa_service.rs
// use serde::{Deserialize, Serialize};
// use reqwest::Client;
// use std::collections::HashMap;
// use uuid::Uuid;
// use crate::error::AppError;
// use crate::services::auth_service::Claims;
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpaRequest {
//     pub input: OpaInput,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpaInput {
//     pub user: OpaUser,
//     pub action: String,
//     pub resource: OpaResource,
//     pub context: HashMap<String, serde_json::Value>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpaUser {
//     pub id: Uuid,
//     pub partner_id: Uuid,
//     pub is_main_partner: bool,
//     pub can_access_all_partners: bool,
//     pub roles: Vec<String>,
//     pub permissions: Vec<String>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpaResource {
//     pub resource_type: String,
//     pub resource_id: Option<String>,
//     pub partner_id: Option<Uuid>,
//     pub attributes: HashMap<String, serde_json::Value>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpaResponse {
//     pub result: OpaDecision,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OpaDecision {
//     pub allow: bool,
//     pub reasons: Vec<String>,
// }
//
// #[derive(Clone)]
// pub struct OpaService {
//     client: Client,
//     opa_url: String,
// }
//
// impl OpaService {
//     pub fn new(opa_url: String) -> Self {
//         Self {
//             client: Client::new(),
//             opa_url,
//         }
//     }
//
//     pub async fn check_permission(
//         &self,
//         claims: &Claims,
//         action: &str,
//         resource: OpaResource,
//     ) -> Result<bool, AppError> {
//         let opa_user = OpaUser {
//             id: claims.sub,
//             partner_id: claims.partner_id,
//             is_main_partner: claims.is_main_partner,
//             can_access_all_partners: claims.can_access_all_partners,
//             roles: claims.roles.clone(),
//             permissions: claims.permissions
//                 .iter()
//                 .map(|p| format!("{}:{}", p.resource, p.action))
//                 .collect(),
//         };
//
//         let opa_input = OpaInput {
//             user: opa_user,
//             action: action.to_string(),
//             resource,
//             context: HashMap::new(),
//         };
//
//         let opa_request = OpaRequest { input: opa_input };
//         println!("{:#?}", opa_request);
//         let response = self.client
//             .post(&format!("{}/v1/data/authz/allow", self.opa_url))
//             .json(&opa_request)
//             .send()
//             .await?;
//
//         if response.status().is_success() {
//             let opa_response: OpaResponse = response.json().await?;
//             Ok(opa_response.result.allow)
//         } else {
//             Ok(false)
//         }
//     }
//
//     pub async fn evaluate_policy(
//         &self,
//         policy_path: &str,
//         input: serde_json::Value,
//     ) -> Result<serde_json::Value, AppError> {
//         let response = self.client
//             .post(&format!("{}/v1/data/{}", self.opa_url, policy_path))
//             .json(&serde_json::json!({ "input": input }))
//             .send()
//             .await?;
//
//         if response.status().is_success() {
//             Ok(response.json().await?)
//         } else {
//             Err(AppError::ExternalService("OPA evaluation failed".into()))
//         }
//     }
// }


// src/services/opa_service.rs - FIXED VERSION
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;
// use uuid::Uuid;
use crate::error::AppError;
use crate::services::auth_service::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpaRequest {
    pub input: OpaInput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpaInput {
    pub user: OpaUser,
    pub action: String,
    pub resource: OpaResource,
    pub context: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpaUser {
    pub id: i32,
    pub partner_id: i32,
    pub is_main_partner: bool,
    pub can_access_all_partners: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpaResource {
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub partner_id: Option<i32>,
    pub attributes: HashMap<String, serde_json::Value>,
}

// ✅ FIX: OPA'nın gerçek response formatı
#[derive(Debug, Serialize, Deserialize)]
pub struct OpaResponse {
    pub result: serde_json::Value,  // ✅ OPA "result" field'ını döndürür
}

// Alternatif: Direkt boolean response için
#[derive(Debug, Serialize, Deserialize)]
pub struct OpaSimpleResponse {
    pub result: bool,
}

#[derive(Clone)]
pub struct OpaService {
    client: Client,
    opa_url: String,
}

impl OpaService {
    pub fn new(opa_url: String) -> Self {
        Self {
            client: Client::new(),
            opa_url,
        }
    }

    pub async fn check_permission(
        &self,
        claims: &Claims,
        action: &str,
        resource: OpaResource,
    ) -> Result<bool, AppError> {
        let opa_user = OpaUser {
            id: claims.sub,
            partner_id: claims.partner_id,
            is_main_partner: claims.is_main_partner,
            can_access_all_partners: claims.can_access_all_partners,
            roles: claims.roles.clone(),
            permissions: claims.permissions
                .iter()
                .map(|p| format!("{}:{}", p.resource, p.action))
                .collect(),
        };

        let opa_input = OpaInput {
            user: opa_user,
            action: action.to_string(),
            resource,
            context: HashMap::new(),
        };

        let opa_request = OpaRequest { input: opa_input };

        // Debug için
        println!("🔍 OPA Request: {:#?}", opa_request);

        let response = self.client
            .post(&format!("{}/v1/data/authz/allow", self.opa_url))
            .json(&opa_request)
            .send()
            .await
            .map_err(|e| {
                eprintln!("❌ OPA request failed: {}", e);
                AppError::ExternalService(format!("OPA request failed: {}", e))
            })?;

        let status = response.status();
        let response_text = response.text().await?;

        // Debug için
        println!("📥 OPA Response Status: {}", status);
        println!("📥 OPA Response Body: {}", response_text);

        if status.is_success() {
            // ✅ FIX: OPA response formatını düzgün parse et
            // OPA şu formatta döner: {"result": true} veya {"result": false}

            // Yöntem 1: JSON Value olarak parse et
            let json: serde_json::Value = serde_json::from_str(&response_text)
                .map_err(|e| {
                    eprintln!("❌ Failed to parse OPA response: {}", e);
                    AppError::ExternalService(format!("Invalid OPA response: {}", e))
                })?;

            // "result" field'ını al
            let result = json.get("result")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            println!("✅ OPA Decision: {}", result);
            Ok(result)

            // Alternatif Yöntem 2: Direkt struct'a parse et
            // let opa_response: OpaSimpleResponse = serde_json::from_str(&response_text)?;
            // Ok(opa_response.result)
        } else {
            eprintln!("❌ OPA returned error status: {} - Body: {}", status, response_text);

            // OPA hata mesajını parse etmeye çalış
            if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                if let Some(error_msg) = error_json.get("message").and_then(|v| v.as_str()) {
                    return Err(AppError::ExternalService(format!("OPA error: {}", error_msg)));
                }
            }

            Ok(false)
        }
    }

    // pub async fn evaluate_policy(
    //     &self,
    //     policy_path: &str,
    //     input: serde_json::Value,
    // ) -> Result<serde_json::Value, AppError> {
    //     let response = self.client
    //         .post(&format!("{}/v1/data/{}", self.opa_url, policy_path))
    //         .json(&serde_json::json!({ "input": input }))
    //         .send()
    //         .await?;
    //
    //     let status = response.status();
    //     let response_text = response.text().await?;
    //
    //     println!("📥 OPA Evaluate Response Status: {}", status);
    //     println!("📥 OPA Evaluate Response Body: {}", response_text);
    //
    //     if status.is_success() {
    //         let json: serde_json::Value = serde_json::from_str(&response_text)?;
    //         Ok(json)
    //     } else {
    //         Err(AppError::ExternalService(format!("OPA evaluation failed: {}", response_text)))
    //     }
    // }

    // ✅ Yeni: OPA'nın sağlığını kontrol et
    pub async fn health_check(&self) -> Result<bool, AppError> {
        let response = self.client
            .get(&format!("{}/health", self.opa_url))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    // ✅ Yeni: OPA'daki verileri kontrol et
    pub async fn check_data(&self) -> Result<serde_json::Value, AppError> {
        let response = self.client
            .get(&format!("{}/v1/data/system", self.opa_url))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(AppError::ExternalService("Failed to get OPA data".into()))
        }
    }
}

// ✅ TEST FONKSİYONU - OPA bağlantısını test etmek için
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_opa_connection() {
        let opa_service = OpaService::new("http://localhost:8181".to_string());

        // Health check
        match opa_service.health_check().await {
            Ok(true) => println!("✅ OPA is healthy"),
            Ok(false) => println!("❌ OPA is unhealthy"),
            Err(e) => println!("❌ OPA connection failed: {}", e),
        }

        // Check data
        match opa_service.check_data().await {
            Ok(data) => println!("✅ OPA Data: {:#?}", data),
            Err(e) => println!("❌ Failed to get OPA data: {}", e),
        }
    }
}