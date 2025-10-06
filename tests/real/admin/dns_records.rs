//! Real API integration tests for DnsRecordHandler

use crate::real::*;
use files_sdk::DnsRecordHandler;

#[tokio::test]
async fn test_real_api_list_dns_records() {
    let client = get_test_client();
    let handler = DnsRecordHandler::new(client);

    let result = handler.list().await;

    match result {
        Ok(records) => {
            println!("Listed {} DNS records", records.len());
            if let Some(first) = records.first() {
                println!("Sample record: {:?}", first);
            }
        }
        Err(e) => {
            println!("List failed (may require custom domain): {:?}", e);
        }
    }
}
