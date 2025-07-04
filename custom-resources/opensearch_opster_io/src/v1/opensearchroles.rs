// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/opensearch-project/opensearch-k8s-operator/opensearch.opster.io/v1/opensearchroles.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// OpensearchRoleSpec defines the desired state of OpensearchRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "opensearch.opster.io", version = "v1", kind = "OpensearchRole", plural = "opensearchroles")]
#[kube(namespaced)]
#[kube(status = "OpensearchRoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct OpensearchRoleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterPermissions")]
    pub cluster_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexPermissions")]
    pub index_permissions: Option<Vec<OpensearchRoleIndexPermissions>>,
    /// LocalObjectReference contains enough information to let you locate the
    /// referenced object inside the same namespace.
    #[serde(rename = "opensearchCluster")]
    pub opensearch_cluster: OpensearchRoleOpensearchCluster,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantPermissions")]
    pub tenant_permissions: Option<Vec<OpensearchRoleTenantPermissions>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpensearchRoleIndexPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedActions")]
    pub allowed_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dls: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fls: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexPatterns")]
    pub index_patterns: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maskedFields")]
    pub masked_fields: Option<Vec<String>>,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpensearchRoleOpensearchCluster {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpensearchRoleTenantPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedActions")]
    pub allowed_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantPatterns")]
    pub tenant_patterns: Option<Vec<String>>,
}

/// OpensearchRoleStatus defines the observed state of OpensearchRole
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpensearchRoleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "existingRole")]
    pub existing_role: Option<bool>,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managedCluster")]
    pub managed_cluster: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

