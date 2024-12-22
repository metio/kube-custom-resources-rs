/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `actions.summerwind.dev` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## actions.summerwind.dev/v1alpha1
- `HorizontalRunnerAutoscaler`
- `RunnerDeployment`
- `RunnerReplicaSet`
- `Runner`
- `RunnerSet`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
