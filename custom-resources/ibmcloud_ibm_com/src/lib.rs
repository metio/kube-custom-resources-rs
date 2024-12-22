/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `ibmcloud.ibm.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## ibmcloud.ibm.com/v1alpha1
- `Composable`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
