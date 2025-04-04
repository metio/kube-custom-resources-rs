// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/mariadb-operator/mariadb-operator/k8s.mariadb.com/v1alpha1/grants.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GrantSpec defines the desired state of Grant
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.mariadb.com", version = "v1alpha1", kind = "Grant", plural = "grants")]
#[kube(namespaced)]
#[kube(status = "GrantStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GrantSpec {
    /// CleanupPolicy defines the behavior for cleaning up a SQL resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cleanupPolicy")]
    pub cleanup_policy: Option<GrantCleanupPolicy>,
    /// Database to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// GrantOption to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grantOption")]
    pub grant_option: Option<bool>,
    /// Host to use in the Grant. It can be localhost, an IP or '%'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// MariaDBRef is a reference to a MariaDB object.
    #[serde(rename = "mariaDbRef")]
    pub maria_db_ref: GrantMariaDbRef,
    /// Privileges to use in the Grant.
    pub privileges: Vec<String>,
    /// RequeueInterval is used to perform requeue reconciliations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requeueInterval")]
    pub requeue_interval: Option<String>,
    /// RetryInterval is the interval used to perform retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryInterval")]
    pub retry_interval: Option<String>,
    /// Table to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    /// Username to use in the Grant.
    pub username: String,
}

/// GrantSpec defines the desired state of Grant
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GrantCleanupPolicy {
    Skip,
    Delete,
}

/// MariaDBRef is a reference to a MariaDB object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrantMariaDbRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// WaitForIt indicates whether the controller using this reference should wait for MariaDB to be ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitForIt")]
    pub wait_for_it: Option<bool>,
}

/// GrantStatus defines the observed state of Grant
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrantStatus {
    /// Conditions for the Grant object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

