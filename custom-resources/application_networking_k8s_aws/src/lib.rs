/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `application-networking.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## application-networking.k8s.aws/v1alpha1
- `AccessLogPolicy`
- `IAMAuthPolicy`
- `ServiceExport`
- `ServiceImport`
- `TargetGroupPolicy`
- `VpcAssociationPolicy`
*/
pub mod v1alpha1;
