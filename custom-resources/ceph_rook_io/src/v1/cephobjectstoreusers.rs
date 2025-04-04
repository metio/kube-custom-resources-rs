// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/rook/rook/ceph.rook.io/v1/cephobjectstoreusers.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// ObjectStoreUserSpec represent the spec of an Objectstoreuser
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephObjectStoreUser", plural = "cephobjectstoreusers")]
#[kube(namespaced)]
#[kube(status = "CephObjectStoreUserStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CephObjectStoreUserSpec {
    /// Additional admin-level capabilities for the Ceph object store user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CephObjectStoreUserCapabilities>,
    /// The namespace where the parent CephCluster and CephObjectStore are found
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNamespace")]
    pub cluster_namespace: Option<String>,
    /// The display name for the ceph users
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    /// ObjectUserQuotaSpec can be used to set quotas for the object store user to limit their usage. See the [Ceph docs](https://docs.ceph.com/en/latest/radosgw/admin/?#quota-management) for more
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quotas: Option<CephObjectStoreUserQuotas>,
    /// The store the user will be created in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectStoreUserCapabilities {
    /// Add capabilities for user to send request to RGW Cache API header. Documented in https://docs.ceph.com/en/latest/radosgw/rgw-cache/#cache-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amz-cache")]
    pub amz_cache: Option<CephObjectStoreUserCapabilitiesAmzCache>,
    /// Add capabilities for user to change bucket index logging. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bilog: Option<CephObjectStoreUserCapabilitiesBilog>,
    /// Admin capabilities to read/write Ceph object store buckets. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<CephObjectStoreUserCapabilitiesBucket>,
    /// Admin capabilities to read/write Ceph object store buckets. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buckets: Option<CephObjectStoreUserCapabilitiesBuckets>,
    /// Add capabilities for user to change data logging. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datalog: Option<CephObjectStoreUserCapabilitiesDatalog>,
    /// Admin capabilities to read/write information about the user. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<CephObjectStoreUserCapabilitiesInfo>,
    /// Add capabilities for user to change metadata logging. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdlog: Option<CephObjectStoreUserCapabilitiesMdlog>,
    /// Admin capabilities to read/write Ceph object store metadata. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CephObjectStoreUserCapabilitiesMetadata>,
    /// Add capabilities for user to change oidc provider. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oidc-provider")]
    pub oidc_provider: Option<CephObjectStoreUserCapabilitiesOidcProvider>,
    /// Add capabilities for user to set rate limiter for user and bucket. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratelimit: Option<CephObjectStoreUserCapabilitiesRatelimit>,
    /// Admin capabilities to read/write roles for user. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<CephObjectStoreUserCapabilitiesRoles>,
    /// Admin capabilities to read/write Ceph object store usage. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<CephObjectStoreUserCapabilitiesUsage>,
    /// Admin capabilities to read/write Ceph object store users. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<CephObjectStoreUserCapabilitiesUser>,
    /// Add capabilities for user to change user policies. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "user-policy")]
    pub user_policy: Option<CephObjectStoreUserCapabilitiesUserPolicy>,
    /// Admin capabilities to read/write Ceph object store users. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<CephObjectStoreUserCapabilitiesUsers>,
    /// Admin capabilities to read/write Ceph object store zones. Documented in https://docs.ceph.com/en/latest/radosgw/admin/?#add-remove-admin-capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<CephObjectStoreUserCapabilitiesZone>,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesAmzCache {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesBilog {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesBucket {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesBuckets {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesDatalog {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesInfo {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesMdlog {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesMetadata {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesOidcProvider {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesRatelimit {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesRoles {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesUsage {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesUser {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesUserPolicy {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesUsers {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// Additional admin-level capabilities for the Ceph object store user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephObjectStoreUserCapabilitiesZone {
    #[serde(rename = "*")]
    KopiumVariant0,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read, write")]
    ReadWrite,
}

/// ObjectUserQuotaSpec can be used to set quotas for the object store user to limit their usage. See the [Ceph docs](https://docs.ceph.com/en/latest/radosgw/admin/?#quota-management) for more
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectStoreUserQuotas {
    /// Maximum bucket limit for the ceph user
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxBuckets")]
    pub max_buckets: Option<i64>,
    /// Maximum number of objects across all the user's buckets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxObjects")]
    pub max_objects: Option<i64>,
    /// Maximum size limit of all objects across all the user's buckets
    /// See https://pkg.go.dev/k8s.io/apimachinery/pkg/api/resource#Quantity for more info.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<IntOrString>,
}

/// ObjectStoreUserStatus represents the status Ceph Object Store Gateway User
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephObjectStoreUserStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<BTreeMap<String, String>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

