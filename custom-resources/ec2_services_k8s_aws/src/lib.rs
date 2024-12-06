/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `ec2.services.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## ec2.services.k8s.aws/v1alpha1
- `DHCPOptions`
- `ElasticIPAddress`
- `Instance`
- `InternetGateway`
- `NATGateway`
- `RouteTable`
- `SecurityGroup`
- `Subnet`
- `TransitGateway`
- `VPCEndpoint`
- `VPC`
*/
pub mod v1alpha1;
