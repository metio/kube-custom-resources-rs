// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbasecollections.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec defines the desired state of the resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseCollection", plural = "couchbasecollections")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CouchbaseCollectionSpec {
    /// MaxTTL defines how long a document is permitted to exist for, without
    /// modification, until it is automatically deleted.  This field takes precedence over
    /// any TTL defined at the bucket level.  This is a default, and maximum
    /// time-to-live and may be set to a lower value by the client.  If the client specifies
    /// a higher value, then it is truncated to the maximum durability.  Documents are
    /// removed by Couchbase, after they have expired, when either accessed, the expiry
    /// pager is run, or the bucket is compacted.  When set to 0, then documents are not
    /// expired by default.  This field must be a duration in the range 0-2147483648s,
    /// defaulting to 0.  More info:
    /// https://golang.org/pkg/time/#ParseDuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTTL")]
    pub max_ttl: Option<String>,
    /// Name specifies the name of the collection.  By default, the metadata.name is
    /// used to define the collection name, however, due to the limited character set,
    /// this field can be used to override the default and provide the full functionality.
    /// Additionally the `metadata.name` field is a DNS label, and thus limited to 63
    /// characters, this field must be used if the name is longer than this limit.
    /// Collection names must be 1-251 characters in length, contain only [a-zA-Z0-9_-%]
    /// and not start with either _ or %.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

