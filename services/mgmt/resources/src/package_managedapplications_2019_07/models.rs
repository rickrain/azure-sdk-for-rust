#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Information about managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The managed application properties."]
    pub properties: ApplicationProperties,
    #[doc = "Plan for the managed application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "The kind of the managed application. Allowed values are MarketPlace and ServiceCatalog."]
    pub kind: String,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl Application {
    pub fn new(properties: ApplicationProperties, kind: String) -> Self {
        Self {
            generic_resource: GenericResource::default(),
            properties,
            plan: None,
            kind,
            identity: None,
        }
    }
}
#[doc = "Managed application artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationArtifact {
    #[doc = "The managed application artifact name."]
    pub name: ApplicationArtifactName,
    #[doc = "The managed application artifact blob uri."]
    pub uri: String,
    #[doc = "The managed application artifact type."]
    #[serde(rename = "type")]
    pub type_: ApplicationArtifactType,
}
impl ApplicationArtifact {
    pub fn new(name: ApplicationArtifactName, uri: String, type_: ApplicationArtifactType) -> Self {
        Self { name, uri, type_ }
    }
}
#[doc = "The managed application artifact name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactName {
    NotSpecified,
    ViewDefinition,
    Authorizations,
    CustomRoleDefinition,
}
#[doc = "The managed application artifact type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactType {
    NotSpecified,
    Template,
    Custom,
}
#[doc = "The managed application provider authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationAuthorization {
    #[doc = "The provider's principal identifier. This is the identity that the provider will use to call ARM to manage the managed application resources."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The provider's role definition identifier. This role will define all the permissions that the provider must have on the managed application's container resource group. This role definition cannot have permission to delete the resource group."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
impl ApplicationAuthorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            role_definition_id,
        }
    }
}
#[doc = "Managed application billing details definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationBillingDetailsDefinition {
    #[doc = "The managed application resource usage Id."]
    #[serde(rename = "resourceUsageId", default, skip_serializing_if = "Option::is_none")]
    pub resource_usage_id: Option<String>,
}
impl ApplicationBillingDetailsDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The application client details to track the entity creating/updating the managed app resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationClientDetails {
    #[doc = "The client Oid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    #[doc = "The client Puid"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub puid: Option<String>,
    #[doc = "The client application Id."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
impl ApplicationClientDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about managed application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinition {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The managed application definition properties."]
    pub properties: ApplicationDefinitionProperties,
}
impl ApplicationDefinition {
    pub fn new(properties: ApplicationDefinitionProperties) -> Self {
        Self {
            generic_resource: GenericResource::default(),
            properties,
        }
    }
}
#[doc = "Application definition artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionArtifact {
    #[doc = "The managed application artifact name."]
    pub name: ApplicationDefinitionArtifactName,
    #[doc = "The managed application definition artifact blob uri."]
    pub uri: String,
    #[doc = "The managed application artifact type."]
    #[serde(rename = "type")]
    pub type_: ApplicationArtifactType,
}
impl ApplicationDefinitionArtifact {
    pub fn new(name: ApplicationDefinitionArtifactName, uri: String, type_: ApplicationArtifactType) -> Self {
        Self { name, uri, type_ }
    }
}
#[doc = "The managed application artifact name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationDefinitionArtifactName {
    NotSpecified,
    ApplicationResourceTemplate,
    CreateUiDefinition,
    MainTemplateParameters,
}
#[doc = "List of managed application definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationDefinitionListResult {
    #[doc = "The array of managed application definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ApplicationDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionProperties {
    #[doc = "The managed application lock level."]
    #[serde(rename = "lockLevel")]
    pub lock_level: ApplicationLockLevel,
    #[doc = "The managed application definition display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A value indicating whether the package is enabled or not."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "The managed application provider authorizations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<ApplicationAuthorization>,
    #[doc = "The collection of managed application artifacts. The portal will use the files specified as artifacts to construct the user experience of creating a managed application from a managed application definition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplicationDefinitionArtifact>,
    #[doc = "The managed application definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The managed application definition package file Uri. Use this element"]
    #[serde(rename = "packageFileUri", default, skip_serializing_if = "Option::is_none")]
    pub package_file_uri: Option<String>,
    #[doc = "The inline main template json which has resources to be provisioned. It can be a JObject or well-formed JSON string."]
    #[serde(rename = "mainTemplate", default, skip_serializing_if = "Option::is_none")]
    pub main_template: Option<serde_json::Value>,
    #[doc = "The createUiDefinition json for the backing template with Microsoft.Solutions/applications resource. It can be a JObject or well-formed JSON string."]
    #[serde(rename = "createUiDefinition", default, skip_serializing_if = "Option::is_none")]
    pub create_ui_definition: Option<serde_json::Value>,
    #[doc = "Managed application notification policy."]
    #[serde(rename = "notificationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<ApplicationNotificationPolicy>,
    #[doc = "Managed application locking policy."]
    #[serde(rename = "lockingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub locking_policy: Option<ApplicationPackageLockingPolicyDefinition>,
    #[doc = "Managed application deployment policy."]
    #[serde(rename = "deploymentPolicy", default, skip_serializing_if = "Option::is_none")]
    pub deployment_policy: Option<ApplicationDeploymentPolicy>,
    #[doc = "Managed application management policy."]
    #[serde(rename = "managementPolicy", default, skip_serializing_if = "Option::is_none")]
    pub management_policy: Option<ApplicationManagementPolicy>,
    #[doc = "The managed application provider policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<ApplicationPolicy>,
}
impl ApplicationDefinitionProperties {
    pub fn new(lock_level: ApplicationLockLevel) -> Self {
        Self {
            lock_level,
            display_name: None,
            is_enabled: None,
            authorizations: Vec::new(),
            artifacts: Vec::new(),
            description: None,
            package_file_uri: None,
            main_template: None,
            create_ui_definition: None,
            notification_policy: None,
            locking_policy: None,
            deployment_policy: None,
            management_policy: None,
            policies: Vec::new(),
        }
    }
}
#[doc = "Managed application deployment policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDeploymentPolicy {
    #[doc = "The deployment mode."]
    #[serde(rename = "deploymentMode")]
    pub deployment_mode: DeploymentMode,
}
impl ApplicationDeploymentPolicy {
    pub fn new(deployment_mode: DeploymentMode) -> Self {
        Self { deployment_mode }
    }
}
#[doc = "Managed application Jit access policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationJitAccessPolicy {
    #[doc = "Whether the JIT access is enabled."]
    #[serde(rename = "jitAccessEnabled")]
    pub jit_access_enabled: bool,
    #[doc = "The Jit approval mode."]
    #[serde(rename = "jitApprovalMode", default, skip_serializing_if = "Option::is_none")]
    pub jit_approval_mode: Option<JitApprovalMode>,
    #[doc = "The JIT approvers"]
    #[serde(rename = "jitApprovers", default, skip_serializing_if = "Vec::is_empty")]
    pub jit_approvers: Vec<JitApproverDefinition>,
    #[doc = "The maximum duration JIT access is granted. This is an ISO8601 time period value."]
    #[serde(rename = "maximumJitAccessDuration", default, skip_serializing_if = "Option::is_none")]
    pub maximum_jit_access_duration: Option<String>,
}
impl ApplicationJitAccessPolicy {
    pub fn new(jit_access_enabled: bool) -> Self {
        Self {
            jit_access_enabled,
            jit_approval_mode: None,
            jit_approvers: Vec::new(),
            maximum_jit_access_duration: None,
        }
    }
}
#[doc = "List of managed applications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationListResult {
    #[doc = "The array of managed applications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ApplicationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application lock level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationLockLevel {
    CanNotDelete,
    ReadOnly,
    None,
}
#[doc = "The management mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationManagementMode {
    NotSpecified,
    Unmanaged,
    Managed,
}
#[doc = "Managed application management policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationManagementPolicy {
    #[doc = "The management mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ApplicationManagementMode>,
}
impl ApplicationManagementPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed application notification endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationNotificationEndpoint {
    #[doc = "The managed application notification endpoint uri."]
    pub uri: String,
}
impl ApplicationNotificationEndpoint {
    pub fn new(uri: String) -> Self {
        Self { uri }
    }
}
#[doc = "Managed application notification policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationNotificationPolicy {
    #[doc = "The managed application notification endpoint."]
    #[serde(rename = "notificationEndpoints")]
    pub notification_endpoints: Vec<ApplicationNotificationEndpoint>,
}
impl ApplicationNotificationPolicy {
    pub fn new(notification_endpoints: Vec<ApplicationNotificationEndpoint>) -> Self {
        Self { notification_endpoints }
    }
}
#[doc = "The application package contact information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageContact {
    #[doc = "The contact name."]
    #[serde(rename = "contactName", default, skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[doc = "The contact email."]
    pub email: String,
    #[doc = "The contact phone number."]
    pub phone: String,
}
impl ApplicationPackageContact {
    pub fn new(email: String, phone: String) -> Self {
        Self {
            contact_name: None,
            email,
            phone,
        }
    }
}
#[doc = "Managed application locking policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPackageLockingPolicyDefinition {
    #[doc = "The deny assignment excluded actions."]
    #[serde(rename = "allowedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_actions: Vec<String>,
    #[doc = "The deny assignment excluded data actions."]
    #[serde(rename = "allowedDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_data_actions: Vec<String>,
}
impl ApplicationPackageLockingPolicyDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The appliance package support URLs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPackageSupportUrls {
    #[doc = "The public azure support URL."]
    #[serde(rename = "publicAzure", default, skip_serializing_if = "Option::is_none")]
    pub public_azure: Option<String>,
    #[doc = "The government cloud support URL."]
    #[serde(rename = "governmentCloud", default, skip_serializing_if = "Option::is_none")]
    pub government_cloud: Option<String>,
}
impl ApplicationPackageSupportUrls {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPatchable {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The managed application properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
    #[doc = "Plan for the managed application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanPatchable>,
    #[doc = "The kind of the managed application. Allowed values are MarketPlace and ServiceCatalog."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl ApplicationPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed application policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPolicy {
    #[doc = "The policy name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The policy definition Id."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The policy parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}
impl ApplicationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[doc = "The managed resource group Id."]
    #[serde(rename = "managedResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[doc = "The fully qualified path of managed application definition Id."]
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[doc = "Name and value pairs that define the managed application parameters. It can be a JObject or a well formed JSON string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Name and value pairs that define the managed application outputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "Provisioning status of the managed application."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Managed application billing details definition."]
    #[serde(rename = "billingDetails", default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<ApplicationBillingDetailsDefinition>,
    #[doc = "Managed application Jit access policy."]
    #[serde(rename = "jitAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub jit_access_policy: Option<ApplicationJitAccessPolicy>,
    #[doc = "The publisher tenant Id."]
    #[serde(rename = "publisherTenantId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_tenant_id: Option<String>,
    #[doc = "The  read-only authorizations property that is retrieved from the application package."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<ApplicationAuthorization>,
    #[doc = "The management mode."]
    #[serde(rename = "managementMode", default, skip_serializing_if = "Option::is_none")]
    pub management_mode: Option<ApplicationManagementMode>,
    #[doc = "The application package contact information."]
    #[serde(rename = "customerSupport", default, skip_serializing_if = "Option::is_none")]
    pub customer_support: Option<ApplicationPackageContact>,
    #[doc = "The appliance package support URLs."]
    #[serde(rename = "supportUrls", default, skip_serializing_if = "Option::is_none")]
    pub support_urls: Option<ApplicationPackageSupportUrls>,
    #[doc = "The collection of managed application artifacts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplicationArtifact>,
    #[doc = "The application client details to track the entity creating/updating the managed app resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ApplicationClientDetails>,
    #[doc = "The application client details to track the entity creating/updating the managed app resource."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<ApplicationClientDetails>,
}
impl ApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationPropertiesPatchable {
    #[doc = "The managed resource group Id."]
    #[serde(rename = "managedResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[doc = "The fully qualified path of managed application definition Id."]
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[doc = "Name and value pairs that define the managed application parameters. It can be a JObject or a well formed JSON string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Name and value pairs that define the managed application outputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "Provisioning status of the managed application."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ApplicationPropertiesPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deployment mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeploymentMode {
    NotSpecified,
    Incremental,
    Complete,
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
#[doc = "Resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "ID of the resource that manages this resource."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "SKU for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl GenericResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "The list of user identities associated with the resource. The user identity dictionary key references will be resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "The Jit approval mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitApprovalMode {
    NotSpecified,
    AutoApprove,
    ManualApprove,
}
#[doc = "JIT approver definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitApproverDefinition {
    #[doc = "The approver service principal Id."]
    pub id: String,
    #[doc = "The approver type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<jit_approver_definition::Type>,
    #[doc = "The approver display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl JitApproverDefinition {
    pub fn new(id: String) -> Self {
        Self {
            id,
            type_: None,
            display_name: None,
        }
    }
}
pub mod jit_approver_definition {
    use super::*;
    #[doc = "The approver type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "group")]
        Group,
    }
}
#[doc = "The JIT authorization policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitAuthorizationPolicies {
    #[doc = "The the principal id that will be granted JIT access."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The role definition id that will be granted to the Principal."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
impl JitAuthorizationPolicies {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            role_definition_id,
        }
    }
}
#[doc = "Information about JIT request definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JitRequestDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Information about JIT request properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JitRequestProperties>,
}
impl JitRequestDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of JIT requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JitRequestDefinitionListResult {
    #[doc = "The array of Jit request definition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JitRequestDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl JitRequestDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about JIT request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JitRequestPatchable {
    #[doc = "Jit request tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl JitRequestPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about JIT request properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestProperties {
    #[doc = "The parent application id."]
    #[serde(rename = "applicationResourceId")]
    pub application_resource_id: String,
    #[doc = "The publisher tenant id."]
    #[serde(rename = "publisherTenantId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_tenant_id: Option<String>,
    #[doc = "The JIT authorization policies."]
    #[serde(rename = "jitAuthorizationPolicies")]
    pub jit_authorization_policies: Vec<JitAuthorizationPolicies>,
    #[doc = "The JIT scheduling policies."]
    #[serde(rename = "jitSchedulingPolicy")]
    pub jit_scheduling_policy: JitSchedulingPolicy,
    #[doc = "Provisioning status of the managed application."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The JIT request state."]
    #[serde(rename = "jitRequestState", default, skip_serializing_if = "Option::is_none")]
    pub jit_request_state: Option<JitRequestState>,
    #[doc = "The application client details to track the entity creating/updating the managed app resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ApplicationClientDetails>,
    #[doc = "The application client details to track the entity creating/updating the managed app resource."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<ApplicationClientDetails>,
}
impl JitRequestProperties {
    pub fn new(
        application_resource_id: String,
        jit_authorization_policies: Vec<JitAuthorizationPolicies>,
        jit_scheduling_policy: JitSchedulingPolicy,
    ) -> Self {
        Self {
            application_resource_id,
            publisher_tenant_id: None,
            jit_authorization_policies,
            jit_scheduling_policy,
            provisioning_state: None,
            jit_request_state: None,
            created_by: None,
            updated_by: None,
        }
    }
}
#[doc = "The JIT request state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitRequestState {
    NotSpecified,
    Pending,
    Approved,
    Denied,
    Failed,
    Canceled,
    Expired,
    Timeout,
}
#[doc = "The JIT scheduling policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitSchedulingPolicy {
    #[doc = "The JIT request scheduling type."]
    #[serde(rename = "type")]
    pub type_: JitSchedulingType,
    pub duration: String,
    #[doc = "The start time of the request."]
    #[serde(rename = "startTime")]
    pub start_time: String,
}
impl JitSchedulingPolicy {
    pub fn new(type_: JitSchedulingType, duration: String, start_time: String) -> Self {
        Self {
            type_,
            duration,
            start_time,
        }
    }
}
#[doc = "The JIT request scheduling type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitSchedulingType {
    NotSpecified,
    Once,
    Recurring,
}
#[doc = "Microsoft.Solutions operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Solutions"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Application, JitRequest, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Plan for the managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[doc = "The plan name."]
    pub name: String,
    #[doc = "The publisher ID."]
    pub publisher: String,
    #[doc = "The product code."]
    pub product: String,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The plan's version."]
    pub version: String,
}
impl Plan {
    pub fn new(name: String, publisher: String, product: String, version: String) -> Self {
        Self {
            name,
            publisher,
            product,
            promotion_code: None,
            version,
        }
    }
}
#[doc = "Plan for the managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanPatchable {
    #[doc = "The plan name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The product code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The plan's version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl PlanPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning status of the managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    NotSpecified,
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Updating,
}
#[doc = "Resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The SKU name."]
    pub name: String,
    #[doc = "The SKU tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The SKU size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The SKU family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The SKU model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "The SKU capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            model: None,
            capacity: None,
        }
    }
}
#[doc = "Represents the user assigned identity that is contained within the UserAssignedIdentities dictionary on ResourceIdentity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedResourceIdentity {
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of user assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl UserAssignedResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
