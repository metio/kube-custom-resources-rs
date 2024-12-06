// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbaserolebindings.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// CouchbaseRoleBindingSpec defines the group of subjects i.e. users, and the
/// role i.e. group they are a member of.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseRoleBinding", plural = "couchbaserolebindings")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct CouchbaseRoleBindingSpec {
    /// CouchbaseGroup being bound to subjects.
    #[serde(rename = "roleRef")]
    pub role_ref: CouchbaseRoleBindingRoleRef,
    /// List of users to bind a role to.
    pub subjects: Vec<CouchbaseRoleBindingSubjects>,
}

/// CouchbaseGroup being bound to subjects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseRoleBindingRoleRef {
    /// Kind of role to use for binding.
    pub kind: CouchbaseRoleBindingRoleRefKind,
    /// Name of role resource to use for binding.
    pub name: String,
}

/// CouchbaseGroup being bound to subjects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseRoleBindingRoleRefKind {
    CouchbaseGroup,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CouchbaseRoleBindingSubjects {
    /// Couchbase user/group kind.
    pub kind: CouchbaseRoleBindingSubjectsKind,
    /// Name of Couchbase user resource.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseRoleBindingSubjectsKind {
    CouchbaseUser,
}

