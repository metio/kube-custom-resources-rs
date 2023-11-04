// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/storageaccounts.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// StorageAccountAdditionalResources holds the additional resources
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageAccountAdditionalResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,
}

/// StorageAccountOutput is the object that contains the output from creating a Storage Account object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageAccountOutput {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionString1")]
    pub connection_string1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionString2")]
    pub connection_string2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageAccountName")]
    pub storage_account_name: Option<String>,
}

/// StorageAccountSpec defines the desired state of Storage
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "StorageAccount", plural = "storageaccounts")]
#[kube(namespaced)]
#[kube(status = "StorageAccountStatus")]
#[kube(schema = "disabled")]
pub struct StorageAccountSpec {
    /// StorageAccountAccessTier enumerates the values for access tier. Only one of the following access tiers may be specified. If none of the following access tiers is specified, the default one is Hot.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessTier")]
    pub access_tier: Option<StorageAccountAccessTier>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataLakeEnabled")]
    pub data_lake_enabled: Option<bool>,
    /// StorageAccountKind enumerates the values for kind. Only one of the following kinds may be specified. If none of the following kinds is specified, the default one is StorageV2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<StorageAccountKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkRule")]
    pub network_rule: Option<StorageAccountNetworkRule>,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    /// StorageAccountSku the SKU of the storage account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<StorageAccountSku>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportsHttpsTrafficOnly")]
    pub supports_https_traffic_only: Option<bool>,
}

/// StorageAccountSpec defines the desired state of Storage
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StorageAccountAccessTier {
    Cool,
    Hot,
}

/// StorageAccountSpec defines the desired state of Storage
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StorageAccountKind {
    BlobStorage,
    BlockBlobStorage,
    FileStorage,
    Storage,
    StorageV2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageAccountNetworkRule {
    /// Bypass - Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Possible values are any combination of Logging|Metrics|AzureServices (For example, "Logging, Metrics"), or None to bypass none of those traffics. Possible values include: 'None', 'Logging', 'Metrics', 'AzureServices'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass: Option<String>,
    /// DefaultAction - Specifies the default action of allow or deny when no other rules match. Possible values include: 'DefaultActionAllow', 'DefaultActionDeny'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultAction")]
    pub default_action: Option<String>,
    /// IPRules - Sets the IP ACL rules
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipRules")]
    pub ip_rules: Option<Vec<StorageAccountNetworkRuleIpRules>>,
    /// VirtualNetworkRules - Sets the virtual network rules
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualNetworkRules")]
    pub virtual_network_rules: Option<Vec<StorageAccountNetworkRuleVirtualNetworkRules>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageAccountNetworkRuleIpRules {
    /// IPAddressOrRange - Specifies the IP or IP range in CIDR format. Only IPV4 address is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressOrRange")]
    pub ip_address_or_range: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageAccountNetworkRuleVirtualNetworkRules {
    /// SubnetId - Resource ID of a subnet, for example: /subscriptions/{subscriptionId}/resourceGroups/{groupName}/providers/Microsoft.Network/virtualNetworks/{vnetName}/subnets/{subnetName}.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetId")]
    pub subnet_id: Option<String>,
}

/// StorageAccountSku the SKU of the storage account.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageAccountSku {
    /// Name - The SKU name. Required for account creation; optional for update. Possible values include: 'Standard_LRS', 'Standard_GRS', 'Standard_RAGRS', 'Standard_ZRS', 'Premium_LRS', 'Premium_ZRS', 'Standard_GZRS', 'Standard_RAGZRS'. For the full list of allowed options, see: https://docs.microsoft.com/en-us/rest/api/storagerp/storageaccounts/create#skuname
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<StorageAccountSkuName>,
}

/// StorageAccountSku the SKU of the storage account.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StorageAccountSkuName {
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(rename = "Standard_GRS")]
    StandardGrs,
    #[serde(rename = "Standard_GZRS")]
    StandardGzrs,
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Standard_RAGRS")]
    StandardRagrs,
    #[serde(rename = "Standard_RAGZRS")]
    StandardRagzrs,
    #[serde(rename = "Standard_ZRS")]
    StandardZrs,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorageAccountStatus {
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
    pub polling_url_kind: Option<StorageAccountStatusPollingUrlKind>,
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
pub enum StorageAccountStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}
