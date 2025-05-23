// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/external-secrets/external-secrets/generators.external-secrets.io/v1alpha1/grafanas.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// GrafanaSpec controls the behavior of the grafana generator.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "generators.external-secrets.io", version = "v1alpha1", kind = "Grafana", plural = "grafanas")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GrafanaSpec {
    /// Auth is the authentication configuration to authenticate
    /// against the Grafana instance.
    pub auth: GrafanaAuth,
    /// ServiceAccount is the configuration for the service account that
    /// is supposed to be generated by the generator.
    #[serde(rename = "serviceAccount")]
    pub service_account: GrafanaServiceAccount,
    /// URL is the URL of the Grafana instance.
    pub url: String,
}

/// Auth is the authentication configuration to authenticate
/// against the Grafana instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAuth {
    /// Basic auth credentials used to authenticate against the Grafana instance.
    /// Note: you need a token which has elevated permissions to create service accounts.
    /// See here for the documentation on basic roles offered by Grafana:
    /// https://grafana.com/docs/grafana/latest/administration/roles-and-permissions/access-control/rbac-fixed-basic-role-definitions/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic: Option<GrafanaAuthBasic>,
    /// A service account token used to authenticate against the Grafana instance.
    /// Note: you need a token which has elevated permissions to create service accounts.
    /// See here for the documentation on basic roles offered by Grafana:
    /// https://grafana.com/docs/grafana/latest/administration/roles-and-permissions/access-control/rbac-fixed-basic-role-definitions/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<GrafanaAuthToken>,
}

/// Basic auth credentials used to authenticate against the Grafana instance.
/// Note: you need a token which has elevated permissions to create service accounts.
/// See here for the documentation on basic roles offered by Grafana:
/// https://grafana.com/docs/grafana/latest/administration/roles-and-permissions/access-control/rbac-fixed-basic-role-definitions/
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAuthBasic {
    /// A basic auth password used to authenticate against the Grafana instance.
    pub password: GrafanaAuthBasicPassword,
    /// A basic auth username used to authenticate against the Grafana instance.
    pub username: String,
}

/// A basic auth password used to authenticate against the Grafana instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAuthBasicPassword {
    /// The key where the token is found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the Secret resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A service account token used to authenticate against the Grafana instance.
/// Note: you need a token which has elevated permissions to create service accounts.
/// See here for the documentation on basic roles offered by Grafana:
/// https://grafana.com/docs/grafana/latest/administration/roles-and-permissions/access-control/rbac-fixed-basic-role-definitions/
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaAuthToken {
    /// The key where the token is found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the Secret resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ServiceAccount is the configuration for the service account that
/// is supposed to be generated by the generator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaServiceAccount {
    /// Name is the name of the service account that will be created by ESO.
    pub name: String,
    /// Role is the role of the service account.
    /// See here for the documentation on basic roles offered by Grafana:
    /// https://grafana.com/docs/grafana/latest/administration/roles-and-permissions/access-control/rbac-fixed-basic-role-definitions/
    pub role: String,
}

