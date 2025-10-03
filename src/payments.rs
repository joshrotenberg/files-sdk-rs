//! Payment operations
//!
//! Payments represent payment transactions for your Files.com account.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};

/// A Payment Line Item entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLineItemEntity {
    /// Line item ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Invoice ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<i64>,

    /// Payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<i64>,
}

/// A Payment entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentEntity {
    /// Payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,

    /// Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Currency (e.g., "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Download URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,

    /// Associated payment line items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_line_items: Option<Vec<PaymentLineItemEntity>>,
}

/// Handler for payment operations
pub struct PaymentHandler {
    client: FilesClient,
}

impl PaymentHandler {
    /// Create a new payment handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List payments
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    ///
    /// # Returns
    /// Tuple of (payments, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, PaymentHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = PaymentHandler::new(client);
    /// let (payments, _) = handler.list(None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<PaymentEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };

        let response = self.client.get_raw(&format!("/payments{}", query)).await?;
        let payments: Vec<PaymentEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((payments, pagination))
    }

    /// Get a specific payment
    ///
    /// # Arguments
    /// * `id` - Payment ID
    pub async fn get(&self, id: i64) -> Result<PaymentEntity> {
        let response = self.client.get_raw(&format!("/payments/{}", id)).await?;
        Ok(serde_json::from_value(response)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = PaymentHandler::new(client);
    }
}
