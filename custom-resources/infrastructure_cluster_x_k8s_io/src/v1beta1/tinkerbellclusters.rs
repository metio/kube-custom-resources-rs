// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tinkerbell/cluster-api-provider-tinkerbell/infrastructure.cluster.x-k8s.io/v1beta1/tinkerbellclusters.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// TinkerbellClusterSpec defines the desired state of TinkerbellCluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "TinkerbellCluster", plural = "tinkerbellclusters")]
#[kube(namespaced)]
#[kube(status = "TinkerbellClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TinkerbellClusterSpec {
    /// ControlPlaneEndpoint is a required field by ClusterAPI v1beta1.
    /// 
    /// See https://cluster-api.sigs.k8s.io/developer/architecture/controllers/cluster.html
    /// for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<TinkerbellClusterControlPlaneEndpoint>,
    /// ImageLookupBaseRegistry is the base Registry URL that is used for pulling images,
    /// if not set, the default will be to use ghcr.io/tinkerbell/cluster-api-provider-tinkerbell.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupBaseRegistry")]
    pub image_lookup_base_registry: Option<String>,
    /// ImageLookupFormat is the URL naming format to use for machine images when
    /// a machine does not specify. When set, this will be used for all cluster machines
    /// unless a machine specifies a different ImageLookupFormat. Supports substitutions
    /// for {{.BaseRegistry}}, {{.OSDistro}}, {{.OSVersion}} and {{.KubernetesVersion}} with
    /// the basse URL, OS distribution, OS version, and kubernetes version, respectively.
    /// BaseRegistry will be the value in ImageLookupBaseRegistry or ghcr.io/tinkerbell/cluster-api-provider-tinkerbell
    /// (the default), OSDistro will be the value in ImageLookupOSDistro or ubuntu (the default),
    /// OSVersion will be the value in ImageLookupOSVersion or default based on the OSDistro
    /// (if known), and the kubernetes version as defined by the packages produced by
    /// kubernetes/release: v1.13.0, v1.12.5-mybuild.1, or v1.17.3. For example, the default
    /// image format of {{.BaseRegistry}}/{{.OSDistro}}-{{.OSVersion}}:{{.KubernetesVersion}}.gz will
    /// attempt to pull the image from that location. See also: https://golang.org/pkg/text/template/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupFormat")]
    pub image_lookup_format: Option<String>,
    /// ImageLookupOSDistro is the name of the OS distro to use when fetching machine images,
    /// if not set it will default to ubuntu.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupOSDistro")]
    pub image_lookup_os_distro: Option<String>,
    /// ImageLookupOSVersion is the version of the OS distribution to use when fetching machine
    /// images. If not set it will default based on ImageLookupOSDistro.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupOSVersion")]
    pub image_lookup_os_version: Option<String>,
}

/// ControlPlaneEndpoint is a required field by ClusterAPI v1beta1.
/// 
/// See https://cluster-api.sigs.k8s.io/developer/architecture/controllers/cluster.html
/// for more details.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellClusterControlPlaneEndpoint {
    /// The hostname on which the API server is serving.
    pub host: String,
    /// The port on which the API server is serving.
    pub port: i32,
}

/// TinkerbellClusterStatus defines the observed state of TinkerbellCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellClusterStatus {
    /// Ready denotes that the cluster (infrastructure) is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

