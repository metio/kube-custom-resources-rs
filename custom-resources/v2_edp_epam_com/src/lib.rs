/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `v2.edp.epam.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## v2.edp.epam.com/v1
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
## v2.edp.epam.com/v1alpha1
- `Template`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
