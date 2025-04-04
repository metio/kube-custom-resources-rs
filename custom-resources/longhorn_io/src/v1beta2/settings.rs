// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/settings.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// The status of the setting.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SettingStatus {
    /// The setting is applied.
    pub applied: bool,
}

