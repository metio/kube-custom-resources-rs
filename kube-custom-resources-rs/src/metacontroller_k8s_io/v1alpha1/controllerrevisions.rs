// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/metacontroller/metacontroller/metacontroller.k8s.io/v1alpha1/controllerrevisions.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2


use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ControllerRevisionChildren {
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    pub kind: String,
    pub names: Vec<String>,
}

