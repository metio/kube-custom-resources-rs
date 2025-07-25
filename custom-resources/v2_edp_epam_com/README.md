<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# v2.edp.epam.com

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `v2.edp.epam.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### v2.edp.epam.com/v1
- `CDPipeline`
- `Stage`
- `CDStageDeploy`
- `CodebaseBranch`
- `CodebaseImageStream`
- `Codebase`
- `GitServer`
- `JiraIssueMetadata`
- `JiraServer`
- `QuickLink`
- `GerritGroupMember`
- `GerritGroup`
- `GerritMergeRequest`
- `GerritProjectAccess`
- `GerritProject`
- `GerritReplicationConfig`
- `Gerrit`
### v2.edp.epam.com/v1alpha1
- `Template`
