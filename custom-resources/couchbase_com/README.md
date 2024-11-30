<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# couchbase.com

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `couchbase.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### couchbase.com/v2
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
