// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-ibmcloud/infrastructure.cluster.x-k8s.io/v1beta1/ibmpowervsclusters.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// IBMPowerVSClusterSpec defines the desired state of IBMPowerVSCluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "IBMPowerVSCluster", plural = "ibmpowervsclusters")]
#[kube(namespaced)]
#[kube(status = "IBMPowerVSClusterStatus")]
#[kube(schema = "disabled")]
pub struct IBMPowerVSClusterSpec {
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<IBMPowerVSClusterControlPlaneEndpoint>,
    /// Network is the reference to the Network to use for this cluster.
    pub network: IBMPowerVSClusterNetwork,
    /// ServiceInstanceID is the id of the power cloud instance where the vsi instance will get deployed.
    #[serde(rename = "serviceInstanceID")]
    pub service_instance_id: String,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterControlPlaneEndpoint {
    /// The hostname on which the API server is serving.
    pub host: String,
    /// The port on which the API server is serving.
    pub port: i32,
}

/// Network is the reference to the Network to use for this cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterNetwork {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource, In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// IBMPowerVSClusterStatus defines the observed state of IBMPowerVSCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterStatus {
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster Important: Run "make" to regenerate code after modifying this file
    pub ready: bool,
}

