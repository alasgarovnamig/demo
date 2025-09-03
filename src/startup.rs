// src/startup.rs - OPA veri yükleme eklentisi
use sea_orm::DatabaseConnection;
use crate::services::opa_data_sync::OpaDataSync;
use tokio::time::{interval, Duration};

/// Uygulama başlangıcında OPA'ya veri yükler
pub async fn initialize_opa_data(
    db: &DatabaseConnection,
    opa_url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let opa_sync = OpaDataSync::new(db.clone(), opa_url);

    // İlk senkronizasyon
    opa_sync.sync_all_data().await?;

    // Periyodik senkronizasyon için background task başlat
    let opa_sync_clone = opa_sync.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(300)); // Her 5 dakikada bir

        loop {
            interval.tick().await;

            if let Err(e) = opa_sync_clone.sync_all_data().await {
                eprintln!("OPA sync error: {}", e);
            }
        }
    });

    Ok(())
}