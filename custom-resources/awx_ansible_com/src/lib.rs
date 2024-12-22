/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `awx.ansible.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## awx.ansible.com/v1beta1
- `AWXBackup`
- `AWXRestore`
- `AWX`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
