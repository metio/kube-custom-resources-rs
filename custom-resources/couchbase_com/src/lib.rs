/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `couchbase.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
