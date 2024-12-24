use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DropEntitlementGrantCondition {
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
}

impl DropEntitlementGrantCondition {
    pub fn new(organization_id: String) -> Self {
        Self {
            organization_id,
            category_id: None,
            campaign_id: None,
        }
    }
}
