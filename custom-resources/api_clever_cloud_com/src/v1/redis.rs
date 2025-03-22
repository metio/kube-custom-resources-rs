// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/CleverCloud/clever-operator/api.clever-cloud.com/v1/redis.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "api.clever-cloud.com", version = "v1", kind = "Redis", plural = "redis")]
#[kube(namespaced)]
#[kube(status = "RedisStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RedisSpec {
    pub instance: RedisInstance,
    pub options: RedisOptions,
    pub organisation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisInstance {
    pub plan: String,
    pub region: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisOptions {
    pub encryption: bool,
    pub version: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RedisOptionsVersion {
    #[serde(rename = "724")]
    r#_724,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addon: Option<String>,
}

