// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-ibmcloud/infrastructure.cluster.x-k8s.io/v1beta1/ibmpowervsmachinetemplates.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// IBMPowerVSMachineTemplateSpec defines the desired state of IBMPowerVSMachineTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "IBMPowerVSMachineTemplate", plural = "ibmpowervsmachinetemplates")]
#[kube(namespaced)]
#[kube(status = "IBMPowerVSMachineTemplateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IBMPowerVSMachineTemplateSpec {
    /// IBMPowerVSMachineTemplateResource holds the IBMPowerVSMachine spec.
    pub template: IBMPowerVSMachineTemplateTemplate,
}

/// IBMPowerVSMachineTemplateResource holds the IBMPowerVSMachine spec.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineTemplateTemplate {
    /// IBMPowerVSMachineSpec defines the desired state of IBMPowerVSMachine.
    pub spec: IBMPowerVSMachineTemplateTemplateSpec,
}

/// IBMPowerVSMachineSpec defines the desired state of IBMPowerVSMachine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineTemplateTemplateSpec {
    /// Image is the reference to the Image from which to create the machine instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<IBMPowerVSMachineTemplateTemplateSpecImage>,
    /// ImageRef is an optional reference to a provider-specific resource that holds
    /// the details for provisioning the Image for a Cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageRef")]
    pub image_ref: Option<IBMPowerVSMachineTemplateTemplateSpecImageRef>,
    /// Memory is Amount of memory allocated (in GB)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// Network is the reference to the Network to use for this instance.
    pub network: IBMPowerVSMachineTemplateTemplateSpecNetwork,
    /// ProcType is the processor type, e.g: dedicated, shared, capped
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "procType")]
    pub proc_type: Option<String>,
    /// Processors is Number of processors allocated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors: Option<String>,
    /// ProviderID is the unique identifier as specified by the cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// ServiceInstanceID is the id of the power cloud instance where the vsi instance will get deployed.
    #[serde(rename = "serviceInstanceID")]
    pub service_instance_id: String,
    /// SSHKey is the name of the SSH key pair provided to the vsi for authenticating users.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshKey")]
    pub ssh_key: Option<String>,
    /// SysType is the System type used to host the vsi.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sysType")]
    pub sys_type: Option<String>,
}

/// Image is the reference to the Image from which to create the machine instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineTemplateTemplateSpecImage {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource,
    /// In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// ImageRef is an optional reference to a provider-specific resource that holds
/// the details for provisioning the Image for a Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineTemplateTemplateSpecImageRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Network is the reference to the Network to use for this instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineTemplateTemplateSpecNetwork {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource,
    /// In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// IBMPowerVSMachineTemplateStatus defines the observed state of IBMPowerVSMachineTemplate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSMachineTemplateStatus {
    /// Capacity defines the resource capacity for this machine.
    /// This value is used for autoscaling from zero operations as defined in:
    /// https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20210310-opt-in-autoscaling-from-zero.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<BTreeMap<String, IntOrString>>,
}

