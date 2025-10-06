//! Real API integration tests for HolidayRegionHandler

use crate::real::*;
use files_sdk::HolidayRegionHandler;

#[tokio::test]
async fn test_real_api_list_holiday_regions() {
    let client = get_test_client();
    let handler = HolidayRegionHandler::new(client);

    let result = handler.list().await;

    match result {
        Ok(regions) => {
            println!("Listed {} holiday regions", regions.len());
            if let Some(first) = regions.first() {
                println!("Sample region: {:?}", first);
            }
        }
        Err(e) => {
            println!("List failed: {:?}", e);
        }
    }
}
