// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tungstenfabric/tf-operator/tf.tungsten.io/v1alpha1/kubemanagers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// KubemanagerSpec is the Spec for the kubemanager API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tf.tungsten.io", version = "v1alpha1", kind = "Kubemanager", plural = "kubemanagers")]
#[kube(namespaced)]
#[kube(status = "KubemanagerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KubemanagerSpec {
    /// PodConfiguration is the common services struct.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonConfiguration")]
    pub common_configuration: Option<KubemanagerCommonConfiguration>,
    /// KubemanagerConfiguration is the configuration for the kubemanager API.
    #[serde(rename = "serviceConfiguration")]
    pub service_configuration: KubemanagerServiceConfiguration,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerCommonConfiguration {
    /// AuthParameters auth parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authParameters")]
    pub auth_parameters: Option<KubemanagerCommonConfigurationAuthParameters>,
    /// OS family
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<String>>,
    /// Kubernetes Cluster Configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<KubemanagerCommonConfigurationLogLevel>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<KubemanagerCommonConfigurationTolerations>>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerCommonConfigurationAuthParameters {
    /// AuthenticationMode auth mode
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authMode")]
    pub auth_mode: Option<KubemanagerCommonConfigurationAuthParametersAuthMode>,
    /// KeystoneAuthParameters keystone parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthParameters")]
    pub keystone_auth_parameters: Option<KubemanagerCommonConfigurationAuthParametersKeystoneAuthParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneSecretName")]
    pub keystone_secret_name: Option<String>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KubemanagerCommonConfigurationAuthParametersAuthMode {
    #[serde(rename = "noauth")]
    Noauth,
    #[serde(rename = "keystone")]
    Keystone,
}

/// KeystoneAuthParameters keystone parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerCommonConfigurationAuthParametersKeystoneAuthParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPassword")]
    pub admin_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPort")]
    pub admin_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminTenant")]
    pub admin_tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminUsername")]
    pub admin_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authProtocol")]
    pub auth_protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectDomainName")]
    pub project_domain_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userDomainName")]
    pub user_domain_name: Option<String>,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KubemanagerCommonConfigurationLogLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "none")]
    None,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerCommonConfigurationTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// KubemanagerConfiguration is the configuration for the kubemanager API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerServiceConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloudOrchestrator")]
    pub cloud_orchestrator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<KubemanagerServiceConfigurationContainers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostNetworkService")]
    pub host_network_service: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipFabricForwarding")]
    pub ip_fabric_forwarding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipFabricSnat")]
    pub ip_fabric_snat: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipFabricSubnets")]
    pub ip_fabric_subnets: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesAPIPort")]
    pub kubernetes_api_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesAPISSLPort")]
    pub kubernetes_apissl_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesAPIServer")]
    pub kubernetes_api_server: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesTokenFile")]
    pub kubernetes_token_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSubnet")]
    pub pod_subnet: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicFIPPool")]
    pub public_fip_pool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceSubnet")]
    pub service_subnet: Option<String>,
}

/// Container defines name, image and command.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerServiceConfigurationContainers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// KubemanagerStatus is the Status for the kubemanager API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configChanged")]
    pub config_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<BTreeMap<String, KubemanagerStatusNodes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubemanagerStatusNodes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

