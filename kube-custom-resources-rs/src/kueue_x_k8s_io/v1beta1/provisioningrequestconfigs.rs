// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/provisioningrequestconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ProvisioningRequestConfigSpec defines the desired state of ProvisioningRequestConfig
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kueue.x-k8s.io", version = "v1beta1", kind = "ProvisioningRequestConfig", plural = "provisioningrequestconfigs")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ProvisioningRequestConfigSpec {
    /// managedResources contains the list of resources managed by the autoscaling.
    /// 
    /// If empty, all resources are considered managed.
    /// 
    /// If not empty, the ProvisioningRequest will contain only the podsets that are
    /// requesting at least one of them.
    /// 
    /// If none of the workloads podsets is requesting at least a managed resource,
    /// the workload is considered ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managedResources")]
    pub managed_resources: Option<Vec<String>>,
    /// Parameters contains all other parameters classes may require.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    /// ProvisioningClassName describes the different modes of provisioning the resources.
    /// Check autoscaling.x-k8s.io ProvisioningRequestSpec.ProvisioningClassName for details.
    #[serde(rename = "provisioningClassName")]
    pub provisioning_class_name: String,
}
