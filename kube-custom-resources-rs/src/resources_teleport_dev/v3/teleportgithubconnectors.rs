// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gravitational/teleport/resources.teleport.dev/v3/teleportgithubconnectors.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// GithubConnector resource definition v3 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v3", kind = "TeleportGithubConnector", plural = "teleportgithubconnectors")]
#[kube(namespaced)]
#[kube(status = "TeleportGithubConnectorStatus")]
#[kube(schema = "disabled")]
pub struct TeleportGithubConnectorSpec {
    /// APIEndpointURL is the URL of the API endpoint of the Github instance this connector is for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_endpoint_url: Option<String>,
    /// ClientID is the Github OAuth app client ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ClientSecret is the Github OAuth app client secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Display is the connector display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// EndpointURL is the URL of the GitHub instance this connector is for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// RedirectURL is the authorization callback URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// TeamsToRoles maps Github team memberships onto allowed roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams_to_roles: Option<Vec<TeleportGithubConnectorTeamsToRoles>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportGithubConnectorTeamsToRoles {
    /// Organization is a Github organization a user belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Roles is a list of allowed logins for this org/team.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Team is a team within the organization a user belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

/// Status defines the observed state of the Teleport resource
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportGithubConnectorStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

