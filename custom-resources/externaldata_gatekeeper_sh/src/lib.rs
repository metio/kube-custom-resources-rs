/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `externaldata.gatekeeper.sh` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## externaldata.gatekeeper.sh/v1alpha1
- `Provider`
## externaldata.gatekeeper.sh/v1beta1
- `Provider`
*/
pub mod v1alpha1;
pub mod v1beta1;
