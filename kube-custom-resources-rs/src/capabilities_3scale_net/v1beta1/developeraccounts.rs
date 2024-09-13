// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale/3scale-operator/capabilities.3scale.net/v1beta1/developeraccounts.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DeveloperAccountSpec defines the desired state of DeveloperAccount
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capabilities.3scale.net", version = "v1beta1", kind = "DeveloperAccount", plural = "developeraccounts")]
#[kube(namespaced)]
#[kube(status = "DeveloperAccountStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DeveloperAccountSpec {
    /// MonthlyBillingEnabled sets the billing status. Defaults to "true", ie., active
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monthlyBillingEnabled")]
    pub monthly_billing_enabled: Option<bool>,
    /// MonthlyChargingEnabled Defaults to "true"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monthlyChargingEnabled")]
    pub monthly_charging_enabled: Option<bool>,
    /// OrgName is the organization name
    #[serde(rename = "orgName")]
    pub org_name: String,
    /// ProviderAccountRef references account provider credentials
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountRef")]
    pub provider_account_ref: Option<DeveloperAccountProviderAccountRef>,
}

/// ProviderAccountRef references account provider credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeveloperAccountProviderAccountRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DeveloperAccountStatus defines the observed state of DeveloperAccount
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeveloperAccountStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountID")]
    pub account_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountState")]
    pub account_state: Option<String>,
    /// Current state of the policy resource. Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creditCardStored")]
    pub credit_card_stored: Option<bool>,
    /// ObservedGeneration reflects the generation of the most recently observed Backend Spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ProviderAccountHost contains the 3scale account's provider URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountHost")]
    pub provider_account_host: Option<String>,
}

