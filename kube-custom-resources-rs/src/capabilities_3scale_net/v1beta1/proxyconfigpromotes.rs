// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale/3scale-operator/capabilities.3scale.net/v1beta1/proxyconfigpromotes.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// ProxyConfigPromoteSpec defines the desired state of ProxyConfigPromote
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capabilities.3scale.net", version = "v1beta1", kind = "ProxyConfigPromote", plural = "proxyconfigpromotes")]
#[kube(namespaced)]
#[kube(status = "ProxyConfigPromoteStatus")]
#[kube(schema = "disabled")]
pub struct ProxyConfigPromoteSpec {
    /// deleteCR  deletes this CR when it has successfully completed the promotion
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteCR")]
    pub delete_cr: Option<bool>,
    /// product CR metadata.name
    #[serde(rename = "productCRName")]
    pub product_cr_name: String,
    /// Environment you wish to promote to, if not present defaults to staging and if set to true promotes to production
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production: Option<bool>,
}

/// ProxyConfigPromoteStatus defines the observed state of ProxyConfigPromote
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProxyConfigPromoteStatus {
    /// Current state of the ProxyConfigPromote resource. Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The latest Version in production
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestProductionVersion")]
    pub latest_production_version: Option<i64>,
    /// The latest Version in staging
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestStagingVersion")]
    pub latest_staging_version: Option<i64>,
    /// The id of the product that has been promoted
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "productId")]
    pub product_id: Option<String>,
}

