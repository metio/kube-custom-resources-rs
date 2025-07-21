<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# operator.victoriametrics.com

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `operator.victoriametrics.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### operator.victoriametrics.com/v1beta1
- `VMAlertmanagerConfig`
- `VMNodeScrape`
- `VMPodScrape`
- `VMProbe`
- `VMRule`
- `VMScrapeConfig`
- `VMServiceScrape`
- `VMStaticScrape`
- `VMUser`
