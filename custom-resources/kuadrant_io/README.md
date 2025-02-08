<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# kuadrant.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `kuadrant.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### kuadrant.io/v1
- `RateLimitPolicy`
### kuadrant.io/v1alpha1
- `DNSRecord`
- `ManagedZone`
### kuadrant.io/v1beta1
- `Kuadrant`
### kuadrant.io/v1beta2
- `RateLimitPolicy`
### kuadrant.io/v1beta3
- `RateLimitPolicy`
