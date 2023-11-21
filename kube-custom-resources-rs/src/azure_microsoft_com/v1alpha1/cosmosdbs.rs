// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/cosmosdbs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// CosmosDBSpec defines the desired state of CosmosDB
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "CosmosDB", plural = "cosmosdbs")]
#[kube(namespaced)]
#[kube(status = "CosmosDBStatus")]
#[kube(schema = "disabled")]
pub struct CosmosDBSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipRules")]
    pub ip_rules: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyVaultToStoreSecrets")]
    pub key_vault_to_store_secrets: Option<String>,
    /// CosmosDBKind enumerates the values for kind. Only one of the following kinds may be specified. If none of the following kinds is specified, the default one is GlobalDocumentDBKind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<CosmosDBKind>,
    /// Location is the Azure location where the CosmosDB exists
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<CosmosDBLocations>>,
    /// CosmosDBProperties the CosmosDBProperties of CosmosDB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CosmosDBProperties>,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualNetworkRules")]
    pub virtual_network_rules: Option<Vec<CosmosDBVirtualNetworkRules>>,
}

/// CosmosDBSpec defines the desired state of CosmosDB
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CosmosDBKind {
    #[serde(rename = "GlobalDocumentDB")]
    GlobalDocumentDb,
    #[serde(rename = "MongoDB")]
    MongoDb,
}

/// CosmosDBLocation defines one or more locations for geo-redundancy and high availability
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CosmosDBLocations {
    #[serde(rename = "failoverPriority")]
    pub failover_priority: i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isZoneRedundant")]
    pub is_zone_redundant: Option<bool>,
    #[serde(rename = "locationName")]
    pub location_name: String,
}

/// CosmosDBProperties the CosmosDBProperties of CosmosDB.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CosmosDBProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<CosmosDBPropertiesCapabilities>>,
    /// DatabaseAccountOfferType - The offer type for the Cosmos DB database account.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseAccountOfferType")]
    pub database_account_offer_type: Option<CosmosDBPropertiesDatabaseAccountOfferType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableMultipleWriteLocations")]
    pub enable_multiple_write_locations: Option<bool>,
    /// IsVirtualNetworkFilterEnabled - Flag to indicate whether to enable/disable Virtual Network ACL rules.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isVirtualNetworkFilterEnabled")]
    pub is_virtual_network_filter_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mongoDBVersion")]
    pub mongo_db_version: Option<String>,
}

/// Capability cosmos DB capability object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CosmosDBPropertiesCapabilities {
    /// Name *CosmosCapability `json:"name,omitempty"`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CosmosDBPropertiesCapabilitiesName>,
}

/// Capability cosmos DB capability object
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CosmosDBPropertiesCapabilitiesName {
    EnableCassandra,
    EnableTable,
    EnableGremlin,
    EnableMongo,
}

/// CosmosDBProperties the CosmosDBProperties of CosmosDB.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CosmosDBPropertiesDatabaseAccountOfferType {
    Standard,
}

/// CosmosDBVirtualNetworkRule virtual Network ACL Rule object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CosmosDBVirtualNetworkRules {
    /// IgnoreMissingVNetServiceEndpoint - Create firewall rule before the virtual network has vnet service endpoint enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreMissingVNetServiceEndpoint")]
    pub ignore_missing_v_net_service_endpoint: Option<bool>,
    /// ID - Resource ID of a subnet, for example: /subscriptions/{subscriptionId}/resourceGroups/{groupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName}.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetID")]
    pub subnet_id: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CosmosDBStatus {
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
    pub polling_url_kind: Option<CosmosDBStatusPollingUrlKind>,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CosmosDBStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}
