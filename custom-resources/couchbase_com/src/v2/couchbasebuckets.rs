// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbasebuckets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// CouchbaseBucketSpec is the specification for a Couchbase bucket resource, and
/// allows the bucket to be customized.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseBucket", plural = "couchbasebuckets")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CouchbaseBucketSpec {
    /// AutoCompaction allows the configuration of auto-compaction settings, including on what
    /// conditions disk space is reclaimed and when it is allowed to run, on a per-bucket basis.
    /// If any of these fields are configured, those that are not configured here will take the value set at the cluster level.
    /// Excluding this field (which is the default), will set the autoCompactionSettings to false and the bucket will use cluster defaults.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoCompaction")]
    pub auto_compaction: Option<CouchbaseBucketAutoCompaction>,
    /// CompressionMode defines how Couchbase server handles document compression.  When
    /// off, documents are stored in memory, and transferred to the client uncompressed.
    /// When passive, documents are stored compressed in memory, and transferred to the
    /// client compressed when requested.  When active, documents are stored compresses
    /// in memory and when transferred to the client.  This field must be "off", "passive"
    /// or "active", defaulting to "passive".  Be aware "off" in YAML 1.2 is a boolean, so
    /// must be quoted as a string in configuration files.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionMode")]
    pub compression_mode: Option<CouchbaseBucketCompressionMode>,
    /// ConflictResolution defines how XDCR handles concurrent write conflicts.  Sequence number
    /// based resolution selects the document with the highest sequence number as the most recent.
    /// Timestamp based resolution selects the document that was written to most recently as the
    /// most recent.  This field must be "seqno" (sequence based), or "lww" (timestamp based),
    /// defaulting to "seqno".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conflictResolution")]
    pub conflict_resolution: Option<CouchbaseBucketConflictResolution>,
    /// EnableFlush defines whether a client can delete all documents in a bucket.
    /// This field defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableFlush")]
    pub enable_flush: Option<bool>,
    /// EnableIndexReplica defines whether indexes for this bucket are replicated.
    /// This field defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableIndexReplica")]
    pub enable_index_replica: Option<bool>,
    /// EvictionPolicy controls how Couchbase handles memory exhaustion.  Value only eviction
    /// flushes documents to disk but maintains document metadata in memory in order to improve
    /// query performance.  Full eviction removes all data from memory after the document is
    /// flushed to disk.  This field must be "valueOnly" or "fullEviction", defaulting to
    /// "valueOnly".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionPolicy")]
    pub eviction_policy: Option<CouchbaseBucketEvictionPolicy>,
    /// HistoryRetention configures settings for bucket history retention and default values for associated collections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "historyRetention")]
    pub history_retention: Option<CouchbaseBucketHistoryRetention>,
    /// IOPriority controls how many threads a bucket has, per pod, to process reads and writes.
    /// This field must be "low" or "high", defaulting to "low".  Modification of this field will
    /// cause a temporary service disruption as threads are restarted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ioPriority")]
    pub io_priority: Option<CouchbaseBucketIoPriority>,
    /// MaxTTL defines how long a document is permitted to exist for, without
    /// modification, until it is automatically deleted.  This is a default and maximum
    /// time-to-live and may be set to a lower value by the client.  If the client specifies
    /// a higher value, then it is truncated to the maximum durability.  Documents are
    /// removed by Couchbase, after they have expired, when either accessed, the expiry
    /// pager is run, or the bucket is compacted.  When set to 0, then documents are not
    /// expired by default.  This field must be a duration in the range 0-2147483648s,
    /// defaulting to 0.  More info:
    /// https://golang.org/pkg/time/#ParseDuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTTL")]
    pub max_ttl: Option<String>,
    /// MemoryQuota is a memory limit to the size of a bucket.  When this limit is exceeded,
    /// documents will be evicted from memory to disk as defined by the eviction policy.  The
    /// memory quota is defined per Couchbase pod running the data service.  This field defaults
    /// to, and must be greater than or equal to 100Mi.  More info:
    /// https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryQuota")]
    pub memory_quota: Option<String>,
    /// MiniumumDurability defines how durable a document write is by default, and can
    /// be made more durable by the client.  This feature enables ACID transactions.
    /// When none, Couchbase server will respond when the document is in memory, it will
    /// become eventually consistent across the cluster.  When majority, Couchbase server will
    /// respond when the document is replicated to at least half of the pods running the
    /// data service in the cluster.  When majorityAndPersistActive, Couchbase server will
    /// respond when the document is replicated to at least half of the pods running the
    /// data service in the cluster and the document has been persisted to disk on the
    /// document master pod.  When persistToMajority, Couchbase server will respond when
    /// the document is replicated and persisted to disk on at least half of the pods running
    /// the data service in the cluster.  This field must be either "none", "majority",
    /// "majorityAndPersistActive" or "persistToMajority", defaulting to "none".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minimumDurability")]
    pub minimum_durability: Option<CouchbaseBucketMinimumDurability>,
    /// Name is the name of the bucket within Couchbase server.  By default the Operator
    /// will use the `metadata.name` field to define the bucket name.  The `metadata.name`
    /// field only supports a subset of the supported character set.  When specified, this
    /// field overrides `metadata.name`.  Legal bucket names have a maximum length of 100
    /// characters and may be composed of any character from "a-z", "A-Z", "0-9" and "-_%\.".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Rank determines the bucket’s place in the order in which the rebalance process
    /// handles the buckets on the cluster. The higher a bucket’s assigned integer
    /// (in relation to the integers assigned other buckets), the sooner in the
    /// rebalance process the bucket is handled. This assignment of rank allows a
    /// cluster’s most mission-critical data to be rebalanced with top priority.
    /// This option is only supported for Couchbase Server 7.6.0+.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// Replicas defines how many copies of documents Couchbase server maintains.  This directly
    /// affects how fault tolerant a Couchbase cluster is.  With a single replica, the cluster
    /// can tolerate one data pod going down and still service requests without data loss.  The
    /// number of replicas also affect memory use.  With a single replica, the effective memory
    /// quota for documents is halved, with two replicas it is one third.  The number of replicas
    /// must be between 0 and 3, defaulting to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// Scopes defines whether the Operator manages scopes for the bucket or not, and
    /// the set of scopes defined for the bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<CouchbaseBucketScopes>,
    /// StorageBackend to be assigned to and used by the bucket. Only valid for Couchbase Server 7.0.0 onward.
    /// Two different backend storage mechanisms can be used - "couchstore" or "magma", defaulting to "couchstore".
    /// Note: "magma" is only valid for Couchbase Server 7.1.0 onward.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageBackend")]
    pub storage_backend: Option<CouchbaseBucketStorageBackend>,
}

/// AutoCompaction allows the configuration of auto-compaction settings, including on what
/// conditions disk space is reclaimed and when it is allowed to run, on a per-bucket basis.
/// If any of these fields are configured, those that are not configured here will take the value set at the cluster level.
/// Excluding this field (which is the default), will set the autoCompactionSettings to false and the bucket will use cluster defaults.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketAutoCompaction {
    /// DatabaseFragmentationThreshold defines triggers for when database compaction should start.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseFragmentationThreshold")]
    pub database_fragmentation_threshold: Option<CouchbaseBucketAutoCompactionDatabaseFragmentationThreshold>,
    /// TimeWindow allows restriction of when compaction can occur.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeWindow")]
    pub time_window: Option<CouchbaseBucketAutoCompactionTimeWindow>,
    /// TombstonePurgeInterval controls how long to wait before purging tombstones.
    /// This field must be in the range 1h-1440h, defaulting to the cluster level value.
    /// More info:  https://golang.org/pkg/time/#ParseDuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tombstonePurgeInterval")]
    pub tombstone_purge_interval: Option<String>,
    /// ViewFragmentationThreshold defines triggers for when view compaction should start.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "viewFragmentationThreshold")]
    pub view_fragmentation_threshold: Option<CouchbaseBucketAutoCompactionViewFragmentationThreshold>,
}

/// DatabaseFragmentationThreshold defines triggers for when database compaction should start.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketAutoCompactionDatabaseFragmentationThreshold {
    /// Percent specifies the level of view fragmentation that must be reached for View compaction to be automatically triggered.
    /// This field must be in the range 0-100, defaulting to the cluster level value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i64>,
    /// Size the level of database fragmentation that must be reached for data compaction to be automatically triggered on the bucket.
    /// More info:
    /// https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// TimeWindow allows restriction of when compaction can occur.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketAutoCompactionTimeWindow {
    /// AbortCompactionOutsideWindow stops compaction processes when the
    /// process moves outside the window, defaulting to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "abortCompactionOutsideWindow")]
    pub abort_compaction_outside_window: Option<bool>,
    /// End is a wallclock time, in the form HH:MM, when a compaction should stop.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Start is a wallclock time, in the form HH:MM, when a compaction is permitted to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// ViewFragmentationThreshold defines triggers for when view compaction should start.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketAutoCompactionViewFragmentationThreshold {
    /// Percent specifies the percentage level of View fragmentation that must be reached for View compaction to be automatically triggered on the bucket
    /// This field must be in the range 0-100, defaulting to the cluster level value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i64>,
    /// Size is the level of View fragmentation that must be reached for view compaction to be automatically triggered on the bucket.
    /// More info:
    /// https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// CouchbaseBucketSpec is the specification for a Couchbase bucket resource, and
/// allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBucketCompressionMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "active")]
    Active,
}

/// CouchbaseBucketSpec is the specification for a Couchbase bucket resource, and
/// allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBucketConflictResolution {
    #[serde(rename = "seqno")]
    Seqno,
    #[serde(rename = "lww")]
    Lww,
}

/// CouchbaseBucketSpec is the specification for a Couchbase bucket resource, and
/// allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBucketEvictionPolicy {
    #[serde(rename = "valueOnly")]
    ValueOnly,
    #[serde(rename = "fullEviction")]
    FullEviction,
}

/// HistoryRetention configures settings for bucket history retention and default values for associated collections.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketHistoryRetention {
    /// Bytes defines how much history an individual vbucket should aim to retain on disk in bytes. This field defaults to 0 and has a minimum working value of 2147483648.
    /// This is only supported on buckets with storageBackend=magma.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    /// CollectionHistoryDefault determines whether history retention is enabled for newly created collections by default. This field defaults to true.
    /// This is only supported on buckets with storageBackend=magma.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectionHistoryDefault")]
    pub collection_history_default: Option<bool>,
    /// Seconds defines how many seconds of history an individual vbucket should aim to retain on disk. This field defaults to 0.
    /// This is only supported on buckets with storageBackend=magma.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}

/// CouchbaseBucketSpec is the specification for a Couchbase bucket resource, and
/// allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBucketIoPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}

/// CouchbaseBucketSpec is the specification for a Couchbase bucket resource, and
/// allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBucketMinimumDurability {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "majority")]
    Majority,
    #[serde(rename = "majorityAndPersistActive")]
    MajorityAndPersistActive,
    #[serde(rename = "persistToMajority")]
    PersistToMajority,
}

/// Scopes defines whether the Operator manages scopes for the bucket or not, and
/// the set of scopes defined for the bucket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketScopes {
    /// Managed defines whether scopes are managed for this bucket.
    /// This field is `false` by default, and the Operator will take no actions that
    /// will affect scopes and collections in this bucket.  The default scope and
    /// collection will be present.  When set to `true`, the Operator will manage
    /// user defined scopes, and optionally, their collections as defined by the
    /// `CouchbaseScope`, `CouchbaseScopeGroup`, `CouchbaseCollection` and
    /// `CouchbaseCollectionGroup` resource documentation.  If this field is set to
    /// `false` while the  already managed, then the Operator will leave whatever
    /// configuration is already present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// Resources is an explicit list of named resources that will be considered
    /// for inclusion in this bucket.  If a resource reference doesn't
    /// match a resource, then no error conditions are raised due to undefined
    /// resource creation ordering and eventual consistency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CouchbaseBucketScopesResources>>,
    /// Selector allows resources to be implicitly considered for inclusion in this
    /// bucket.  More info:
    /// https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#labelselector-v1-meta
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CouchbaseBucketScopesSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketScopesResources {
    /// Kind indicates the kind of resource that is being referenced.  A scope
    /// can only reference `CouchbaseScope` and `CouchbaseScopeGroup`
    /// resource kinds.  This field defaults to `CouchbaseScope` if not
    /// specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<CouchbaseBucketScopesResourcesKind>,
    /// Name is the name of the Kubernetes resource name that is being referenced.
    /// Legal scope names have a maximum length of 251
    /// characters and may be composed of any character from "a-z", "A-Z", "0-9" and "_-%".
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBucketScopesResourcesKind {
    CouchbaseScope,
    CouchbaseScopeGroup,
}

/// Selector allows resources to be implicitly considered for inclusion in this
/// bucket.  More info:
/// https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.28/#labelselector-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketScopesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CouchbaseBucketScopesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBucketScopesSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// CouchbaseBucketSpec is the specification for a Couchbase bucket resource, and
/// allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBucketStorageBackend {
    #[serde(rename = "couchstore")]
    Couchstore,
    #[serde(rename = "magma")]
    Magma,
}

