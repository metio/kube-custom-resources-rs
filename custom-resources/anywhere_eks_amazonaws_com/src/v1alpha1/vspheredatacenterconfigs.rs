// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/vspheredatacenterconfigs.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// VSphereDatacenterConfigSpec defines the desired state of VSphereDatacenterConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "VSphereDatacenterConfig", plural = "vspheredatacenterconfigs")]
#[kube(namespaced)]
#[kube(status = "VSphereDatacenterConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereDatacenterConfigSpec {
    pub datacenter: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomains")]
    pub failure_domains: Option<Vec<VSphereDatacenterConfigFailureDomains>>,
    pub insecure: bool,
    pub network: String,
    pub server: String,
    pub thumbprint: String,
}

/// FailureDomain defines the list of failure domains to spread the VMs across.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereDatacenterConfigFailureDomains {
    /// ComputeCluster is the name or inventory path of the computecluster in which the VM is created/located
    #[serde(rename = "computeCluster")]
    pub compute_cluster: String,
    /// Datastore is the name or inventory path of the datastore in which the VM is created/located
    pub datastore: String,
    /// Folder is the name or inventory path of the folder in which the the VM is created/located
    pub folder: String,
    /// Name is used as a unique identifier for each failure domain.
    pub name: String,
    /// Network is the name or inventory path of the network which will be added to the VM
    pub network: String,
    /// ResourcePool is the name or inventory path of the resource pool in which the VM is created/located
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,
}

/// VSphereDatacenterConfigStatus defines the observed state of VSphereDatacenterConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereDatacenterConfigStatus {
    /// FailureMessage indicates that there is a fatal problem reconciling the
    /// state, and will be set to a descriptive error message.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// SpecValid is set to true if vspheredatacenterconfig is validated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specValid")]
    pub spec_valid: Option<bool>,
}

