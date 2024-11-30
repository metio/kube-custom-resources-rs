<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# k8s.mariadb.com

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `k8s.mariadb.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### k8s.mariadb.com/v1alpha1
- `Backup`
- `Connection`
- `Database`
- `Grant`
- `MariaDB`
- `MaxScale`
- `Restore`
- `SqlJob`
- `User`
