// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/aws-application-networking-k8s/application-networking.k8s.aws/v1alpha1/vpcassociationpolicies.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VpcAssociationPolicySpec defines the desired state of VpcAssociationPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "application-networking.k8s.aws", version = "v1alpha1", kind = "VpcAssociationPolicy", plural = "vpcassociationpolicies")]
#[kube(namespaced)]
#[kube(status = "VpcAssociationPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VpcAssociationPolicySpec {
    /// AssociateWithVpc indicates whether the VpcServiceNetworkAssociation should be created for the current VPC of k8s cluster. 
    ///  This value will be considered true by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associateWithVpc")]
    pub associate_with_vpc: Option<bool>,
    /// SecurityGroupIds defines the security groups enforced on the VpcServiceNetworkAssociation. Security groups does not take effect if AssociateWithVpc is set to false. 
    ///  For more details, please check the VPC Lattice documentation https://docs.aws.amazon.com/vpc-lattice/latest/ug/security-groups.html
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// TargetRef points to the kubernetes Gateway resource that will have this policy attached. 
    ///  This field is following the guidelines of Kubernetes Gateway API policy attachment.
    #[serde(rename = "targetRef")]
    pub target_ref: VpcAssociationPolicyTargetRef,
}

/// TargetRef points to the kubernetes Gateway resource that will have this policy attached. 
///  This field is following the guidelines of Kubernetes Gateway API policy attachment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VpcAssociationPolicyTargetRef {
    /// Group is the group of the target resource.
    pub group: String,
    /// Kind is kind of the target resource.
    pub kind: String,
    /// Name is the name of the target resource.
    pub name: String,
    /// Namespace is the namespace of the referent. When unspecified, the local namespace is inferred. Even when policy targets a resource in a different namespace, it MUST only apply to traffic originating from the same namespace as the policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// VpcAssociationPolicyStatus defines the observed state of VpcAssociationPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VpcAssociationPolicyStatus {
    /// Conditions describe the current conditions of the VpcAssociationPolicy. 
    ///  Implementations should prefer to express Policy conditions using the `PolicyConditionType` and `PolicyConditionReason` constants so that operators and tools can converge on a common vocabulary to describe VpcAssociationPolicy state. 
    ///  Known condition types are: 
    ///  * "Accepted"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

