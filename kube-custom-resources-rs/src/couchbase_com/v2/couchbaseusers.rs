// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbaseusers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// CouchbaseUserSpec allows the specification of Couchbase user configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseUser", plural = "couchbaseusers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct CouchbaseUserSpec {
    /// The domain which provides user authentication.
    #[serde(rename = "authDomain")]
    pub auth_domain: CouchbaseUserAuthDomain,
    /// Name of Kubernetes secret with password for Couchbase domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authSecret")]
    pub auth_secret: Option<String>,
    /// Full Name of Couchbase user.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fullName")]
    pub full_name: Option<String>,
    /// Username of the couchbase user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// CouchbaseUserSpec allows the specification of Couchbase user configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseUserAuthDomain {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "external")]
    External,
}

