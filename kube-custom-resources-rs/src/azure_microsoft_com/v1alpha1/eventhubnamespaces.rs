// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/eventhubnamespaces.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// EventhubNamespaceSpec defines the desired state of EventhubNamespace
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "EventhubNamespace", plural = "eventhubnamespaces")]
#[kube(namespaced)]
#[kube(status = "EventhubNamespaceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct EventhubNamespaceSpec {
    /// INSERT ADDITIONAL SPEC FIELDS - desired state of cluster Important: Run "make" to regenerate code after modifying this file
    pub location: String,
    /// EventhubNamespaceNetworkRule defines the namespace network rule
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkRule")]
    pub network_rule: Option<EventhubNamespaceNetworkRule>,
    /// EventhubNamespaceProperties defines the namespace properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventhubNamespaceProperties>,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    /// EventhubNamespaceSku defines the sku
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<EventhubNamespaceSku>,
}

/// EventhubNamespaceNetworkRule defines the namespace network rule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EventhubNamespaceNetworkRule {
    /// DefaultAction defined as a string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultAction")]
    pub default_action: Option<String>,
    /// IPRules - List of IpRules
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipRules")]
    pub ip_rules: Option<Vec<EventhubNamespaceNetworkRuleIpRules>>,
    /// VirtualNetworkRules - List VirtualNetwork Rules
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualNetworkRules")]
    pub virtual_network_rules: Option<Vec<EventhubNamespaceNetworkRuleVirtualNetworkRules>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EventhubNamespaceNetworkRuleIpRules {
    /// IPMask - IPv4 address 1.1.1.1 or CIDR notation 1.1.0.0/24
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipMask")]
    pub ip_mask: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EventhubNamespaceNetworkRuleVirtualNetworkRules {
    /// IgnoreMissingVnetServiceEndpoint - Value that indicates whether to ignore missing VNet Service Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreMissingServiceEndpoint")]
    pub ignore_missing_service_endpoint: Option<bool>,
    /// Subnet - Full Resource ID of Virtual Network Subnet
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetId")]
    pub subnet_id: Option<String>,
}

/// EventhubNamespaceProperties defines the namespace properties
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EventhubNamespaceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isAutoInflateEnabled")]
    pub is_auto_inflate_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kafkaEnabled")]
    pub kafka_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumThroughputUnits")]
    pub maximum_throughput_units: Option<i32>,
}

/// EventhubNamespaceSku defines the sku
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EventhubNamespaceSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EventhubNamespaceStatus {
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
    pub polling_url_kind: Option<EventhubNamespaceStatusPollingUrlKind>,
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
pub enum EventhubNamespaceStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

