<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# slinky.slurm.net

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `slinky.slurm.net` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### slinky.slurm.net/v1alpha1
- `Cluster`
- `NodeSet`
### slinky.slurm.net/v1beta1
- `NodeSet`
