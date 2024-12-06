// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/amazon-vpc-resource-controller-k8s/vpcresources.k8s.aws/v1beta1/securitygrouppolicies.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// SecurityGroupPolicySpec defines the desired state of SecurityGroupPolicy
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "vpcresources.k8s.aws", version = "v1beta1", kind = "SecurityGroupPolicy", plural = "securitygrouppolicies")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SecurityGroupPolicySpec {
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null
    /// label selector matches no objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSelector")]
    pub pod_selector: Option<SecurityGroupPolicyPodSelector>,
    /// GroupIds contains the list of security groups that will be applied to the network interface of the pod matching the criteria.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroups")]
    pub security_groups: Option<SecurityGroupPolicySecurityGroups>,
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null
    /// label selector matches no objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountSelector")]
    pub service_account_selector: Option<SecurityGroupPolicyServiceAccountSelector>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityGroupPolicyPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SecurityGroupPolicyPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityGroupPolicyPodSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// GroupIds contains the list of security groups that will be applied to the network interface of the pod matching the criteria.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityGroupPolicySecurityGroups {
    /// Groups is the list of EC2 Security Groups Ids that need to be applied to the ENI of a Pod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupIds")]
    pub group_ids: Option<Vec<String>>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityGroupPolicyServiceAccountSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SecurityGroupPolicyServiceAccountSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecurityGroupPolicyServiceAccountSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

