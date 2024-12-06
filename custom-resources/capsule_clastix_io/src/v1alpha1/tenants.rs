// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/clastix/capsule/capsule.clastix.io/v1alpha1/tenants.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// TenantSpec defines the desired state of Tenant.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "capsule.clastix.io", version = "v1alpha1", kind = "Tenant", plural = "tenants")]
#[kube(status = "TenantStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct TenantSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalRoleBindings")]
    pub additional_role_bindings: Option<Vec<TenantAdditionalRoleBindings>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRegistries")]
    pub container_registries: Option<TenantContainerRegistries>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalServiceIPs")]
    pub external_service_i_ps: Option<TenantExternalServiceIPs>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClasses")]
    pub ingress_classes: Option<TenantIngressClasses>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressHostnames")]
    pub ingress_hostnames: Option<TenantIngressHostnames>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limitRanges")]
    pub limit_ranges: Option<Vec<TenantLimitRanges>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceQuota")]
    pub namespace_quota: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespacesMetadata")]
    pub namespaces_metadata: Option<TenantNamespacesMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkPolicies")]
    pub network_policies: Option<Vec<TenantNetworkPolicies>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// OwnerSpec defines tenant owner name and kind.
    pub owner: TenantOwner,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceQuotas")]
    pub resource_quotas: Option<Vec<TenantResourceQuotas>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "servicesMetadata")]
    pub services_metadata: Option<TenantServicesMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClasses")]
    pub storage_classes: Option<TenantStorageClasses>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantAdditionalRoleBindings {
    #[serde(rename = "clusterRoleName")]
    pub cluster_role_name: String,
    /// kubebuilder:validation:Minimum=1
    pub subjects: Vec<TenantAdditionalRoleBindingsSubjects>,
}

/// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantAdditionalRoleBindingsSubjects {
    /// APIGroup holds the API group of the referenced subject. Defaults to "" for ServiceAccount subjects. Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount". If the Authorizer does not recognized the kind value, the Authorizer should report an error.
    pub kind: String,
    /// Name of the object being referenced.
    pub name: String,
    /// Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantContainerRegistries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRegex")]
    pub allowed_regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantExternalServiceIPs {
    pub allowed: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantIngressClasses {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRegex")]
    pub allowed_regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantIngressHostnames {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRegex")]
    pub allowed_regex: Option<String>,
}

/// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantLimitRanges {
    /// Limits is the list of LimitRangeItem objects that are enforced.
    pub limits: Vec<TenantLimitRangesLimits>,
}

/// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantLimitRangesLimits {
    /// Default resource requirement limit value by resource name if resource limit is omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<BTreeMap<String, IntOrString>>,
    /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultRequest")]
    pub default_request: Option<BTreeMap<String, IntOrString>>,
    /// Max usage constraints on this kind by resource name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<BTreeMap<String, IntOrString>>,
    /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxLimitRequestRatio")]
    pub max_limit_request_ratio: Option<BTreeMap<String, IntOrString>>,
    /// Min usage constraints on this kind by resource name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<BTreeMap<String, IntOrString>>,
    /// Type of resource that this limit applies to.
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNamespacesMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalAnnotations")]
    pub additional_annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalLabels")]
    pub additional_labels: Option<BTreeMap<String, String>>,
}

/// NetworkPolicySpec provides the specification of a NetworkPolicy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPolicies {
    /// egress is a list of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<TenantNetworkPoliciesEgress>>,
    /// ingress is a list of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<TenantNetworkPoliciesIngress>>,
    /// podSelector selects the pods to which this NetworkPolicy object applies. The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods. In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace.
    #[serde(rename = "podSelector")]
    pub pod_selector: TenantNetworkPoliciesPodSelector,
    /// policyTypes is a list of rule types that the NetworkPolicy relates to. Valid options are ["Ingress"], ["Egress"], or ["Ingress", "Egress"]. If this field is not specified, it will default based on the existence of ingress or egress rules; policies that contain an egress section are assumed to affect egress, and all policies (whether or not they contain an ingress section) are assumed to affect ingress. If you want to write an egress-only policy, you must explicitly specify policyTypes [ "Egress" ]. Likewise, if you want to write a policy that specifies that no egress is allowed, you must specify a policyTypes value that include "Egress" (since such a policy would not include an egress section and would otherwise default to just [ "Ingress" ]). This field is beta-level in 1.8
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyTypes")]
    pub policy_types: Option<Vec<String>>,
}

/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to. This type is beta-level in 1.8
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgress {
    /// ports is a list of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<TenantNetworkPoliciesEgressPorts>>,
    /// to is a list of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all destinations (traffic not restricted by destination). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the to list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<TenantNetworkPoliciesEgressTo>>,
}

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgressPorts {
    /// endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPort")]
    pub end_port: Option<i32>,
    /// port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    /// protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgressTo {
    /// ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipBlock")]
    pub ip_block: Option<TenantNetworkPoliciesEgressToIpBlock>,
    /// namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces. 
    ///  If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<TenantNetworkPoliciesEgressToNamespaceSelector>,
    /// podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods. 
    ///  If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSelector")]
    pub pod_selector: Option<TenantNetworkPoliciesEgressToPodSelector>,
}

/// ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgressToIpBlock {
    /// cidr is a string representing the IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    pub cidr: String,
    /// except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub except: Option<Vec<String>>,
}

/// namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces. 
///  If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgressToNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TenantNetworkPoliciesEgressToNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgressToNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods. 
///  If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgressToPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TenantNetworkPoliciesEgressToPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesEgressToPodSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and from.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngress {
    /// from is a list of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the from list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<TenantNetworkPoliciesIngressFrom>>,
    /// ports is a list of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<TenantNetworkPoliciesIngressPorts>>,
}

/// NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngressFrom {
    /// ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipBlock")]
    pub ip_block: Option<TenantNetworkPoliciesIngressFromIpBlock>,
    /// namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces. 
    ///  If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<TenantNetworkPoliciesIngressFromNamespaceSelector>,
    /// podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods. 
    ///  If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSelector")]
    pub pod_selector: Option<TenantNetworkPoliciesIngressFromPodSelector>,
}

/// ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngressFromIpBlock {
    /// cidr is a string representing the IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    pub cidr: String,
    /// except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub except: Option<Vec<String>>,
}

/// namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces. 
///  If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngressFromNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TenantNetworkPoliciesIngressFromNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngressFromNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods. 
///  If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngressFromPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TenantNetworkPoliciesIngressFromPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngressFromPodSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// NetworkPolicyPort describes a port to allow traffic on
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesIngressPorts {
    /// endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endPort")]
    pub end_port: Option<i32>,
    /// port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    /// protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// podSelector selects the pods to which this NetworkPolicy object applies. The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods. In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TenantNetworkPoliciesPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantNetworkPoliciesPodSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// OwnerSpec defines tenant owner name and kind.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TenantOwner {
    pub kind: TenantOwnerKind,
    pub name: String,
}

/// OwnerSpec defines tenant owner name and kind.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TenantOwnerKind {
    User,
    Group,
}

/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantResourceQuotas {
    /// hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard: Option<BTreeMap<String, IntOrString>>,
    /// scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scopeSelector")]
    pub scope_selector: Option<TenantResourceQuotasScopeSelector>,
    /// A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantResourceQuotasScopeSelector {
    /// A list of scope selector requirements by scope of the resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TenantResourceQuotasScopeSelectorMatchExpressions>>,
}

/// A scoped-resource selector requirement is a selector that contains values, a scope name, and an operator that relates the scope name and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantResourceQuotasScopeSelectorMatchExpressions {
    /// Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist.
    pub operator: String,
    /// The name of the scope that the selector applies to.
    #[serde(rename = "scopeName")]
    pub scope_name: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantServicesMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalAnnotations")]
    pub additional_annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalLabels")]
    pub additional_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantStorageClasses {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRegex")]
    pub allowed_regex: Option<String>,
}

/// TenantStatus defines the observed state of Tenant.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TenantStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    pub size: i64,
}

