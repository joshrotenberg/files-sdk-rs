//! Invoice operations
//!
//! Invoices represent billing line items for your Files.com account.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};

/// An Invoice Line Item entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItemEntity {
    /// Line item ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type (e.g., "invoice")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Service end date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_end_at: Option<String>,

    /// Service start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_start_at: Option<String>,

    /// Plan name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// Site name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
}

/// An Account Line Item entity (Invoice)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLineItemEntity {
    /// Line item ID
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

    /// Associated invoice line items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_items: Option<Vec<InvoiceLineItemEntity>>,
}

/// Handler for invoice operations
pub struct InvoiceHandler {
    client: FilesClient,
}

impl InvoiceHandler {
    /// Create a new invoice handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List invoices
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    ///
    /// # Returns
    /// Tuple of (invoices, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, InvoiceHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = InvoiceHandler::new(client);
    /// let (invoices, _) = handler.list(None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<AccountLineItemEntity>, PaginationInfo)> {
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

        let response = self.client.get_raw(&format!("/invoices{}", query)).await?;
        let invoices: Vec<AccountLineItemEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((invoices, pagination))
    }

    /// Get a specific invoice
    ///
    /// # Arguments
    /// * `id` - Invoice ID
    pub async fn get(&self, id: i64) -> Result<AccountLineItemEntity> {
        let response = self.client.get_raw(&format!("/invoices/{}", id)).await?;
        Ok(serde_json::from_value(response)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = InvoiceHandler::new(client);
    }
}
