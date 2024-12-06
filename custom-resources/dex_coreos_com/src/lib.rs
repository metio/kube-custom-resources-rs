/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `dex.coreos.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## dex.coreos.com/v1
- `AuthCode`
- `AuthRequest`
- `Connector`
- `DeviceRequest`
- `DeviceToken`
- `OAuth2Client`
- `OfflineSessions`
- `Password`
- `RefreshToken`
- `SigningKey`
*/
pub mod v1;
