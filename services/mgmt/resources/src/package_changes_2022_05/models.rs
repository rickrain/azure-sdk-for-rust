#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Details about the change resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeAttributes {
    #[doc = "The ARM correlation ID of the change resource"]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The time the change(s) on the target resource ocurred"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "The number of changes this resource captures"]
    #[serde(rename = "changesCount", default, skip_serializing_if = "Option::is_none")]
    pub changes_count: Option<i64>,
    #[doc = "The GUID of the previous snapshot"]
    #[serde(rename = "previousResourceSnapshotId", default, skip_serializing_if = "Option::is_none")]
    pub previous_resource_snapshot_id: Option<String>,
    #[doc = "The GUID of the new snapshot"]
    #[serde(rename = "newResourceSnapshotId", default, skip_serializing_if = "Option::is_none")]
    pub new_resource_snapshot_id: Option<String>,
}
impl ChangeAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An individual change on the target resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeBase {
    #[doc = "The type of change that occurred"]
    #[serde(rename = "propertyChangeType", default, skip_serializing_if = "Option::is_none")]
    pub property_change_type: Option<change_base::PropertyChangeType>,
    #[doc = "The entity that made the change"]
    #[serde(rename = "changeCategory", default, skip_serializing_if = "Option::is_none")]
    pub change_category: Option<change_base::ChangeCategory>,
    #[doc = "The target resource property value before the change"]
    #[serde(rename = "previousValue", default, skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,
    #[doc = "The target resource property value after the change"]
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
}
impl ChangeBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod change_base {
    use super::*;
    #[doc = "The type of change that occurred"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyChangeType {
        Insert,
        Update,
        Remove,
    }
    #[doc = "The entity that made the change"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeCategory {
        User,
        System,
    }
}
#[doc = "The properties of a change"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeProperties {
    #[doc = "The fully qualified ID of the target resource that was changed"]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "The namespace and type of the resource"]
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[doc = "The type of change that was captured in the resource"]
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<change_properties::ChangeType>,
    #[doc = "Details about the change resource"]
    #[serde(rename = "changeAttributes", default, skip_serializing_if = "Option::is_none")]
    pub change_attributes: Option<ChangeAttributes>,
    #[doc = "A dictionary with changed property name as a key and the change details as the value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<ChangesDictionary>,
}
impl ChangeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod change_properties {
    use super::*;
    #[doc = "The type of change that was captured in the resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        Update,
        Delete,
        Create,
    }
}
#[doc = "The list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeResourceListResult {
    #[doc = "The link used to get the next page of Change Resources"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChangeResourceResult>,
}
impl ChangeResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Change Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeResourceResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a change"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChangeProperties>,
}
impl ChangeResourceResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dictionary with changed property name as a key and the change details as the value"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangesDictionary {}
impl ChangesDictionary {
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
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
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
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
