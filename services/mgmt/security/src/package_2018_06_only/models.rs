#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft Defender for Cloud is provided in two pricing tiers: free and standard, with the standard tier available with a trial period. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pricing {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Pricing properties for the relevant scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PricingProperties>,
}
impl Pricing {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of pricing configurations response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingList {
    #[doc = "List of pricing configurations"]
    pub value: Vec<Pricing>,
}
impl PricingList {
    pub fn new(value: Vec<Pricing>) -> Self {
        Self { value }
    }
}
#[doc = "Pricing properties for the relevant scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PricingProperties {
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard, with the standard tier available with a trial period. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
    #[serde(rename = "pricingTier")]
    pub pricing_tier: pricing_properties::PricingTier,
    #[doc = "The duration left for the subscriptions free trial period - in ISO 8601 format (e.g. P3Y6M4DT12H30M5S)."]
    #[serde(rename = "freeTrialRemainingTime", default, skip_serializing_if = "Option::is_none")]
    pub free_trial_remaining_time: Option<String>,
}
impl PricingProperties {
    pub fn new(pricing_tier: pricing_properties::PricingTier) -> Self {
        Self {
            pricing_tier,
            free_trial_remaining_time: None,
        }
    }
}
pub mod pricing_properties {
    use super::*;
    #[doc = "The pricing tier value. Microsoft Defender for Cloud is provided in two pricing tiers: free and standard, with the standard tier available with a trial period. The standard tier offers advanced security capabilities, while the free tier offers basic security features."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PricingTier {
        Free,
        Standard,
    }
}
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
