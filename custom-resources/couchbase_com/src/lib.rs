/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# couchbase_com

Custom resources in this crate belong to the `couchbase.com` group. The following versions and custom resources are available:

## couchbase.com/v2
- `CouchbaseAutoscaler`
- `CouchbaseBackupRestore`
- `CouchbaseBackup`
- `CouchbaseBucket`
- `CouchbaseCluster`
- `CouchbaseCollectionGroup`
- `CouchbaseCollection`
- `CouchbaseEphemeralBucket`
- `CouchbaseGroup`
- `CouchbaseMemcachedBucket`
- `CouchbaseMigrationReplication`
- `CouchbaseReplication`
- `CouchbaseRoleBinding`
- `CouchbaseScopeGroup`
- `CouchbaseScope`
- `CouchbaseUser`
*/
pub mod v2;
