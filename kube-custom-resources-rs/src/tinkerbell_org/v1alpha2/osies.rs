// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tinkerbell/tink/tinkerbell.org/v1alpha2/osies.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tinkerbell.org", version = "v1alpha2", kind = "OSIE", plural = "osies")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct OSIESpec {
    /// InitrdURL is a URL to an initrd image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initrdUrl")]
    pub initrd_url: Option<String>,
    /// KernelURL is a URL to a kernel image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kernelUrl")]
    pub kernel_url: Option<String>,
}

