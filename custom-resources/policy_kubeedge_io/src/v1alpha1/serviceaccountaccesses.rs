// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubeedge/kubeedge/policy.kubeedge.io/v1alpha1/serviceaccountaccesses.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// Spec represents the specification of rbac.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "policy.kubeedge.io", version = "v1alpha1", kind = "ServiceAccountAccess", plural = "serviceaccountaccesses")]
#[kube(namespaced)]
#[kube(status = "ServiceAccountAccessStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ServiceAccountAccessSpec {
    /// AccessClusterRoleBinding represents rbac ClusterRoleBinding plus detailed ClusterRole info.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessClusterRoleBinding")]
    pub access_cluster_role_binding: Option<Vec<ServiceAccountAccessAccessClusterRoleBinding>>,
    /// AccessRoleBinding represents rbac rolebinding plus detailed role info.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessRoleBinding")]
    pub access_role_binding: Option<Vec<ServiceAccountAccessAccessRoleBinding>>,
    /// ServiceAccount is one-to-one corresponding relations with the serviceaccountaccess.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<ServiceAccountAccessServiceAccount>,
    /// ServiceAccountUID is the uid of serviceaccount.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountUid")]
    pub service_account_uid: Option<String>,
}

/// AccessClusterRoleBinding represents rbac ClusterRoleBinding plus detailed ClusterRole info.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessClusterRoleBinding {
    /// ClusterRoleBinding represents rbac ClusterRoleBinding.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterRoleBinding")]
    pub cluster_role_binding: Option<ServiceAccountAccessAccessClusterRoleBindingClusterRoleBinding>,
    /// Rules contains role rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ServiceAccountAccessAccessClusterRoleBindingRules>>,
}

/// ClusterRoleBinding represents rbac ClusterRoleBinding.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessClusterRoleBindingClusterRoleBinding {
    /// APIVersion defines the versioned schema of this representation of an object.
    /// Servers should convert recognized schemas to the latest internal value, and
    /// may reject unrecognized values.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents.
    /// Servers may infer this from the endpoint the client submits requests to.
    /// Cannot be updated.
    /// In CamelCase.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServiceAccountAccessAccessClusterRoleBindingClusterRoleBindingMetadata>,
    /// RoleRef can only reference a ClusterRole in the global namespace.
    /// If the RoleRef cannot be resolved, the Authorizer must return an error.
    /// This field is immutable.
    #[serde(rename = "roleRef")]
    pub role_ref: ServiceAccountAccessAccessClusterRoleBindingClusterRoleBindingRoleRef,
    /// Subjects holds references to the objects the role applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<ServiceAccountAccessAccessClusterRoleBindingClusterRoleBindingSubjects>>,
}

/// Standard object's metadata.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessClusterRoleBindingClusterRoleBindingMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// RoleRef can only reference a ClusterRole in the global namespace.
/// If the RoleRef cannot be resolved, the Authorizer must return an error.
/// This field is immutable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessClusterRoleBindingClusterRoleBindingRoleRef {
    /// APIGroup is the group for the resource being referenced
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference,
/// or a value for non-objects such as user and group names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessClusterRoleBindingClusterRoleBindingSubjects {
    /// APIGroup holds the API group of the referenced subject.
    /// Defaults to "" for ServiceAccount subjects.
    /// Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount".
    /// If the Authorizer does not recognized the kind value, the Authorizer should report an error.
    pub kind: String,
    /// Name of the object being referenced.
    pub name: String,
    /// Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty
    /// the Authorizer should report an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// PolicyRule holds information that describes a policy rule, but does not contain information
/// about who the rule applies to or which namespace the rule applies to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessClusterRoleBindingRules {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of
    /// the enumerated resources in any API group will be allowed. "" represents the core API group and "*" represents all API groups.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroups")]
    pub api_groups: Option<Vec<String>>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path
    /// Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding.
    /// Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonResourceURLs")]
    pub non_resource_ur_ls: Option<Vec<String>>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceNames")]
    pub resource_names: Option<Vec<String>>,
    /// Resources is a list of resources this rule applies to. '*' represents all resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs.
    pub verbs: Vec<String>,
}

/// AccessRoleBinding represents rbac rolebinding plus detailed role info.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessRoleBinding {
    /// RoleBinding represents rbac rolebinding.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleBinding")]
    pub role_binding: Option<ServiceAccountAccessAccessRoleBindingRoleBinding>,
    /// Rules contains role rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ServiceAccountAccessAccessRoleBindingRules>>,
}

/// RoleBinding represents rbac rolebinding.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessRoleBindingRoleBinding {
    /// APIVersion defines the versioned schema of this representation of an object.
    /// Servers should convert recognized schemas to the latest internal value, and
    /// may reject unrecognized values.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents.
    /// Servers may infer this from the endpoint the client submits requests to.
    /// Cannot be updated.
    /// In CamelCase.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Standard object's metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServiceAccountAccessAccessRoleBindingRoleBindingMetadata>,
    /// RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace.
    /// If the RoleRef cannot be resolved, the Authorizer must return an error.
    /// This field is immutable.
    #[serde(rename = "roleRef")]
    pub role_ref: ServiceAccountAccessAccessRoleBindingRoleBindingRoleRef,
    /// Subjects holds references to the objects the role applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<ServiceAccountAccessAccessRoleBindingRoleBindingSubjects>>,
}

/// Standard object's metadata.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessRoleBindingRoleBindingMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace.
/// If the RoleRef cannot be resolved, the Authorizer must return an error.
/// This field is immutable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessRoleBindingRoleBindingRoleRef {
    /// APIGroup is the group for the resource being referenced
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference,
/// or a value for non-objects such as user and group names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessRoleBindingRoleBindingSubjects {
    /// APIGroup holds the API group of the referenced subject.
    /// Defaults to "" for ServiceAccount subjects.
    /// Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount".
    /// If the Authorizer does not recognized the kind value, the Authorizer should report an error.
    pub kind: String,
    /// Name of the object being referenced.
    pub name: String,
    /// Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty
    /// the Authorizer should report an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// PolicyRule holds information that describes a policy rule, but does not contain information
/// about who the rule applies to or which namespace the rule applies to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessAccessRoleBindingRules {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of
    /// the enumerated resources in any API group will be allowed. "" represents the core API group and "*" represents all API groups.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroups")]
    pub api_groups: Option<Vec<String>>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path
    /// Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding.
    /// Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonResourceURLs")]
    pub non_resource_ur_ls: Option<Vec<String>>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceNames")]
    pub resource_names: Option<Vec<String>>,
    /// Resources is a list of resources this rule applies to. '*' represents all resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs.
    pub verbs: Vec<String>,
}

/// ServiceAccount is one-to-one corresponding relations with the serviceaccountaccess.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessServiceAccount {
    /// APIVersion defines the versioned schema of this representation of an object.
    /// Servers should convert recognized schemas to the latest internal value, and
    /// may reject unrecognized values.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted.
    /// Can be overridden at the pod level.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "automountServiceAccountToken")]
    pub automount_service_account_token: Option<bool>,
    /// ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images
    /// in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets
    /// can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet.
    /// More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<ServiceAccountAccessServiceAccountImagePullSecrets>>,
    /// Kind is a string value representing the REST resource this object represents.
    /// Servers may infer this from the endpoint the client submits requests to.
    /// Cannot be updated.
    /// In CamelCase.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServiceAccountAccessServiceAccountMetadata>,
    /// Secrets is a list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use.
    /// Pods are only limited to this list if this service account has a "kubernetes.io/enforce-mountable-secrets" annotation set to "true".
    /// This field should not be used to find auto-generated service account token secrets for use outside of pods.
    /// Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created.
    /// More info: https://kubernetes.io/docs/concepts/configuration/secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<ObjectReference>>,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessServiceAccountImagePullSecrets {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Drop `kubebuilder:default` when controller-gen doesn't need it https://github.com/kubernetes-sigs/kubebuilder/issues/3896.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Standard object's metadata.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessServiceAccountMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Status represents the node list which store the rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceAccountAccessStatus {
    /// NodeList represents the node name which store the rules.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeList")]
    pub node_list: Option<Vec<String>>,
}

