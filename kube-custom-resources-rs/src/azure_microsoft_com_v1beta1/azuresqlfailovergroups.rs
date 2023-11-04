// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1beta1/azuresqlfailovergroups.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// AzureSqlFailoverGroupSpec defines the desired state of AzureSqlFailoverGroup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "azure.microsoft.com", version = "v1beta1", kind = "AzureSqlFailoverGroup", plural = "azuresqlfailovergroups")]
#[kube(namespaced)]
#[kube(status = "AzureSqlFailoverGroupStatus")]
#[kube(schema = "disabled")]
pub struct AzureSqlFailoverGroupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "SecondaryServerSubscriptionId")]
    pub secondary_server_subscription_id: Option<String>,
    #[serde(rename = "databaseList")]
    pub database_list: Vec<String>,
    /// TODO: This field should be a ptr as it must not be specified if the failover policy is Manual, TODO: but is required when the policy is Automatic
    #[serde(rename = "failoverGracePeriod")]
    pub failover_grace_period: i32,
    /// ReadWriteEndpointFailoverPolicy - wraps https://godoc.org/github.com/Azure/azure-sdk-for-go/services/preview/sql/mgmt/v3.0/sql#ReadWriteEndpointFailoverPolicy
    #[serde(rename = "failoverPolicy")]
    pub failover_policy: AzureSqlFailoverGroupFailoverPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyVaultToStoreSecrets")]
    pub key_vault_to_store_secrets: Option<String>,
    /// Important: Run "make" to regenerate code after modifying this file
    pub location: String,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[serde(rename = "secondaryServer")]
    pub secondary_server: String,
    #[serde(rename = "secondaryServerResourceGroup")]
    pub secondary_server_resource_group: String,
    pub server: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subscriptionId")]
    pub subscription_id: Option<String>,
}

/// AzureSqlFailoverGroupSpec defines the desired state of AzureSqlFailoverGroup
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AzureSqlFailoverGroupFailoverPolicy {
    Automatic,
    Manual,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AzureSqlFailoverGroupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containsUpdate")]
    pub contains_update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedProvisioning")]
    pub failed_provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flattenedSecrets")]
    pub flattened_secrets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrl")]
    pub polling_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrlKind")]
    pub polling_url_kind: Option<AzureSqlFailoverGroupStatusPollingUrlKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specHash")]
    pub spec_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AzureSqlFailoverGroupStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}
