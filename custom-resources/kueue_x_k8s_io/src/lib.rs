/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `kueue.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## kueue.x-k8s.io/v1alpha1
- `Cohort`
- `MultiKueueCluster`
- `MultiKueueConfig`
## kueue.x-k8s.io/v1beta1
- `AdmissionCheck`
- `ClusterQueue`
- `Cohort`
- `LocalQueue`
- `MultiKueueCluster`
- `MultiKueueConfig`
- `ProvisioningRequestConfig`
- `ResourceFlavor`
- `WorkloadPriorityClass`
- `Workload`
## kueue.x-k8s.io/v1beta2
- `AdmissionCheck`
- `ClusterQueue`
- `Cohort`
- `LocalQueue`
- `MultiKueueCluster`
- `MultiKueueConfig`
- `ProvisioningRequestConfig`
- `ResourceFlavor`
- `WorkloadPriorityClass`
- `Workload`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
