// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/berops/claudie/claudie.io/v1beta1/inputmanifests.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Specification of the desired behaviour of the InputManifest
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "claudie.io", version = "v1beta1", kind = "InputManifest", plural = "inputmanifests")]
#[kube(namespaced)]
#[kube(status = "InputManifestStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct InputManifestSpec {
    /// Kubernetes list of Kubernetes cluster this manifest will manage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<InputManifestKubernetes>,
    /// LoadBalancers list of loadbalancer clusters the Kubernetes clusters may use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancers")]
    pub load_balancers: Option<InputManifestLoadBalancers>,
    /// NodePool is a map of dynamic nodepools and static nodepools which will be used to
    /// form kubernetes or loadbalancer clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodePools")]
    pub node_pools: Option<InputManifestNodePools>,
    /// Providers list of defined cloud provider configuration
    /// that will be used while infrastructure provisioning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<InputManifestProviders>>,
}

/// Kubernetes list of Kubernetes cluster this manifest will manage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestKubernetes {
    /// List of Kubernetes clusters Claudie will create.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<InputManifestKubernetesClusters>>,
}

/// Collection of data used to define a Kubernetes cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestKubernetesClusters {
    /// General information about a proxy used to build a K8s cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "installationProxy")]
    pub installation_proxy: Option<InputManifestKubernetesClustersInstallationProxy>,
    /// Name of the Kubernetes cluster. Each cluster will have a random hash appended to the name, so the whole name will be of format <name>-<hash>.
    pub name: String,
    /// Network range for the VPN of the cluster. The value should be defined in format A.B.C.D/mask.
    pub network: String,
    /// List of nodepool names this cluster will use.
    pub pools: InputManifestKubernetesClustersPools,
    /// Version should be defined in format vX.Y. In terms of supported versions of Kubernetes,
    /// Claudie follows kubeone releases and their supported versions.
    /// The current kubeone version used in Claudie is 1.8.1.
    /// To see the list of supported versions, please refer to kubeone documentation.
    /// https://docs.kubermatic.com/kubeone/v1.8/architecture/compatibility/supported-versions/
    pub version: String,
}

/// General information about a proxy used to build a K8s cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestKubernetesClustersInstallationProxy {
    /// Endpoint defines the proxy endpoint. If undefined, the default value is http://proxy.claudie.io:8880.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Mode defines if the proxy mode (on/off/default). If undefined, the default mode is used.
    pub mode: String,
}

/// List of nodepool names this cluster will use.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestKubernetesClustersPools {
    /// List of nodepool names, that will represent compute nodes.
    pub compute: Vec<String>,
    /// List of nodepool names, that will represent control plane nodes.
    pub control: Vec<String>,
}

/// LoadBalancers list of loadbalancer clusters the Kubernetes clusters may use.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestLoadBalancers {
    /// A list of load balancers clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<InputManifestLoadBalancersClusters>>,
    /// List of roles loadbalancers use to forward the traffic. Single role can be used in multiple loadbalancer clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<InputManifestLoadBalancersRoles>>,
}

/// Collection of data used to define a loadbalancer cluster. Defines loadbalancer clusters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestLoadBalancersClusters {
    /// Specification of the loadbalancer's DNS record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<InputManifestLoadBalancersClustersDns>,
    /// Name of the loadbalancer.
    pub name: String,
    /// List of nodepool names this loadbalancer will use. Remember, that nodepools defined
    /// in nodepools are only "blueprints". The actual nodepool will be created once referenced here.
    pub pools: Vec<String>,
    /// List of roles the loadbalancer uses.
    pub roles: Vec<String>,
    /// Name of the Kubernetes cluster targeted by this loadbalancer.
    #[serde(rename = "targetedK8s")]
    pub targeted_k8s: String,
}

/// Specification of the loadbalancer's DNS record.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestLoadBalancersClustersDns {
    /// DNS zone inside of which the records will be created. GCP/AWS/OCI/Azure/Cloudflare/Hetzner DNS zone is accepted
    #[serde(rename = "dnsZone")]
    pub dns_zone: String,
    /// Custom hostname for your A record. If left empty, the hostname will be a random hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Name of provider to be used for creating an A record entry in defined DNS zone.
    pub provider: String,
}

/// Role defines a concrete loadbalancer configuration. Single loadbalancer can have multiple roles.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InputManifestLoadBalancersRoles {
    /// Name of the role. Used as a reference in clusters.
    pub name: String,
    /// Port of the incoming traffic on the loadbalancer.
    pub port: i32,
    /// Protocol of the rule. Allowed values are: tcp, udp.
    pub protocol: InputManifestLoadBalancersRolesProtocol,
    /// Defines nodepools of the targeted K8s cluster, from which nodes will be used for loadbalancing.
    #[serde(rename = "targetPools")]
    pub target_pools: Vec<String>,
    /// Port where loadbalancer forwards the traffic.
    #[serde(rename = "targetPort")]
    pub target_port: i32,
}

/// Role defines a concrete loadbalancer configuration. Single loadbalancer can have multiple roles.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InputManifestLoadBalancersRolesProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

/// NodePool is a map of dynamic nodepools and static nodepools which will be used to
/// form kubernetes or loadbalancer clusters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePools {
    /// Dynamic nodepools define nodepools dynamically created by Claudie.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<Vec<InputManifestNodePoolsDynamic>>,
    /// Static nodepools define nodepools of already existing nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "static")]
    pub r#static: Option<Vec<InputManifestNodePoolsStatic>>,
}

/// DynamicNodePool List of dynamically to-be-created nodepools of not yet existing machines, used for Kubernetes or loadbalancer clusters.
/// These are only blueprints, and will only be created per reference in kubernetes or loadBalancer clusters.
/// 
/// 
/// E.g. if the nodepool isn't used, it won't even be created. Or if the same nodepool is used in two different clusters,
/// it will be created twice. In OOP analogy, a dynamic nodepool would be a class
/// that would get instantiated N >= 0 times depending on which clusters reference it.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsDynamic {
    /// User defined annotations for this nodepool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Autoscaler configuration for this nodepool. Mutually exclusive with count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoscaler: Option<InputManifestNodePoolsDynamicAutoscaler>,
    /// Number of the nodes in the nodepool. Mutually exclusive with autoscaler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// OS image of the machine. Currently, only Ubuntu 22.04 AMD64 images are supported.
    pub image: String,
    /// User defined labels for this nodepool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// MachineSpec further describe the properties of the selected server type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineSpec")]
    pub machine_spec: Option<InputManifestNodePoolsDynamicMachineSpec>,
    /// Name of the nodepool. Each nodepool will have a random hash appended to the name, so the whole name will be of format <name>-<hash>.
    pub name: String,
    /// Collection of provider data to be used while creating the nodepool.
    #[serde(rename = "providerSpec")]
    pub provider_spec: InputManifestNodePoolsDynamicProviderSpec,
    /// 	Type of the machines in the nodepool. Currently, only AMD64 machines are supported.
    #[serde(rename = "serverType")]
    pub server_type: String,
    /// Size of the storage disk on the nodes in the nodepool in GB. The OS disk is created automatically
    /// with predefined size of 100GB for kubernetes nodes and 50GB for Loadbalancer nodes.
    /// The value must be either -1 (no disk is created), or >= 50. If no value is specified, 50 is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageDiskSize")]
    pub storage_disk_size: Option<i32>,
    /// User defined taints for this nodepool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<InputManifestNodePoolsDynamicTaints>>,
}

/// Autoscaler configuration for this nodepool. Mutually exclusive with count.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsDynamicAutoscaler {
    /// Maximum number of nodes in nodepool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// Minimum number of nodes in nodepool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

/// MachineSpec further describe the properties of the selected server type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsDynamicMachineSpec {
    /// CpuCount specifies the number of CPU cores to be used.
    #[serde(rename = "cpuCount")]
    pub cpu_count: i64,
    pub memory: i64,
}

/// Collection of provider data to be used while creating the nodepool.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsDynamicProviderSpec {
    /// Name of the provider instance specified in providers
    pub name: String,
    /// Region of the nodepool.
    pub region: String,
    /// Zone of the nodepool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsDynamicTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// StaticNodePool defines nodepool of already existing nodes, managed outside of Claudie.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsStatic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the nodepool.
    pub name: String,
    /// List of static nodes for a particular static nodepool.
    pub nodes: Vec<InputManifestNodePoolsStaticNodes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<InputManifestNodePoolsStaticTaints>>,
}

/// StaticNode defines a single static node for a particular static nodepool.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsStaticNodes {
    /// Endpoint under which Claudie will access this node.
    pub endpoint: String,
    /// Secret reference to the private key of the node.
    #[serde(rename = "secretRef")]
    pub secret_ref: InputManifestNodePoolsStaticNodesSecretRef,
    /// Username with root access. Used in SSH connection also.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Secret reference to the private key of the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsStaticNodesSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestNodePoolsStaticTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Providers list of defined cloud provider configuration
/// that will be used while infrastructure provisioning.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InputManifestProviders {
    /// Name is the name of the provider specification. It has to be unique across all providers.
    pub name: String,
    /// ProviderType type of a provider.
    /// A list of available providers can be found at https://docs.claudie.io/v0.8.1/input-manifest/providers/aws/
    #[serde(rename = "providerType")]
    pub provider_type: InputManifestProvidersProviderType,
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret
    /// in any namespace
    #[serde(rename = "secretRef")]
    pub secret_ref: InputManifestProvidersSecretRef,
    /// External templates for building the cluster infrastructure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<InputManifestProvidersTemplates>,
}

/// Providers list of defined cloud provider configuration
/// that will be used while infrastructure provisioning.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InputManifestProvidersProviderType {
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "hetzner")]
    Hetzner,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "oci")]
    Oci,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "cloudflare")]
    Cloudflare,
    #[serde(rename = "hetznerdns")]
    Hetznerdns,
    #[serde(rename = "genesiscloud")]
    Genesiscloud,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret
/// in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestProvidersSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// External templates for building the cluster infrastructure.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestProvidersTemplates {
    pub path: String,
    pub repository: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Most recently observed status of the InputManifest
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<BTreeMap<String, InputManifestStatusClusters>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InputManifestStatusClusters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

