// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/hashicorp/terraform-cloud-operator/app.terraform.io/v1alpha2/workspaces.yaml --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// WorkspaceSpec defines the desired state of Workspace.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "app.terraform.io", version = "v1alpha2", kind = "Workspace", plural = "workspaces")]
#[kube(namespaced)]
#[kube(status = "WorkspaceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct WorkspaceSpec {
    /// Terraform Cloud Agents allow Terraform Cloud to communicate with isolated, private, or on-premises infrastructure.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/agents
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "agentPool")]
    pub agent_pool: Option<WorkspaceAgentPool>,
    /// Allows a destroy plan to be created and applied.
    /// Default: `true`.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings#destruction-and-deletion
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowDestroyPlan")]
    pub allow_destroy_plan: Option<bool>,
    /// Define either change will be applied automatically(auto) or require an operator to confirm(manual).
    /// Must be one of the following values: `auto`, `manual`.
    /// Default: `manual`.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings#auto-apply-and-manual-apply
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyMethod")]
    pub apply_method: Option<String>,
    /// Workspace description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Terraform Environment variables for all plans and applies in this workspace.
    /// Variables defined within a workspace always overwrite variables from variable sets that have the same type and the same key.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/variables
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/variables#environment-variables
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "environmentVariables")]
    pub environment_variables: Option<Vec<WorkspaceEnvironmentVariables>>,
    /// Define where the Terraform code will be executed.
    /// Must be one of the following values: `agent`, `local`, `remote`.
    /// Default: `remote`.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings#execution-mode
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionMode")]
    pub execution_mode: Option<String>,
    /// Workspace name.
    pub name: String,
    /// Notifications allow you to send messages to other applications based on run and workspace events.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/notifications
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<WorkspaceNotifications>>,
    /// Organization name where the Workspace will be created.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/users-teams-organizations/organizations
    pub organization: String,
    /// Projects let you organize your workspaces into groups.
    /// Default: default organization project.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/tutorials/cloud/projects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<WorkspaceProject>,
    /// Remote state access between workspaces.
    /// By default, new workspaces in Terraform Cloud do not allow other workspaces to access their state.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/state#accessing-state-from-other-workspaces
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteStateSharing")]
    pub remote_state_sharing: Option<WorkspaceRemoteStateSharing>,
    /// Run tasks allow Terraform Cloud to interact with external systems at specific points in the Terraform Cloud run lifecycle.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/run-tasks
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runTasks")]
    pub run_tasks: Option<Vec<WorkspaceRunTasks>>,
    /// Run triggers allow you to connect this workspace to one or more source workspaces.
    /// These connections allow runs to queue automatically in this workspace on successful apply of runs in any of the source workspaces.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/run-triggers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runTriggers")]
    pub run_triggers: Option<Vec<WorkspaceRunTriggers>>,
    /// SSH key used to clone Terraform modules.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/ssh-keys
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshKey")]
    pub ssh_key: Option<WorkspaceSshKey>,
    /// Workspace tags are used to help identify and group together workspaces.
    /// Tags must be one or more characters; can include letters, numbers, colons, hyphens, and underscores; and must begin and end with a letter or number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Terraform Cloud workspaces can only be accessed by users with the correct permissions.
    /// You can manage permissions for a workspace on a per-team basis.
    /// When a workspace is created, only the owners team and teams with the "manage workspaces" permission can access it,
    /// with full admin permissions. These teams' access can't be removed from a workspace.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/access
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teamAccess")]
    pub team_access: Option<Vec<WorkspaceTeamAccess>>,
    /// Terraform variables for all plans and applies in this workspace.
    /// Variables defined within a workspace always overwrite variables from variable sets that have the same type and the same key.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/variables
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/variables#terraform-variables
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terraformVariables")]
    pub terraform_variables: Option<Vec<WorkspaceTerraformVariables>>,
    /// The version of Terraform to use for this workspace.
    /// If not specified, the latest available version will be used.
    /// Must match pattern: `^\\d{1}\\.\\d{1,2}\\.\\d{1,2}$`
    /// More information:
    ///   - https://www.terraform.io/cloud-docs/workspaces/settings#terraform-version
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terraformVersion")]
    pub terraform_version: Option<String>,
    /// API Token to be used for API calls.
    pub token: WorkspaceToken,
    /// Settings for the workspace's VCS repository, enabling the UI/VCS-driven run workflow.
    /// Omit this argument to utilize the CLI-driven and API-driven workflows, where runs are not driven by webhooks on your VCS provider.
    /// More information:
    ///   - https://www.terraform.io/cloud-docs/run/ui
    ///   - https://www.terraform.io/cloud-docs/vcs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionControl")]
    pub version_control: Option<WorkspaceVersionControl>,
    /// The directory where Terraform will execute, specified as a relative path from the root of the configuration directory.
    /// More information:
    ///   - https://www.terraform.io/cloud-docs/workspaces/settings#terraform-working-directory
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workingDirectory")]
    pub working_directory: Option<String>,
}

/// Terraform Cloud Agents allow Terraform Cloud to communicate with isolated, private, or on-premises infrastructure.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/agents
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceAgentPool {
    /// Agent Pool ID.
    /// Must match pattern: `^apool-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Agent Pool name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Variables let you customize configurations, modify Terraform's behavior, and store information like provider credentials.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/variables
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceEnvironmentVariables {
    /// Description of the variable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parse this field as HashiCorp Configuration Language (HCL). This allows you to interpolate values at runtime.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hcl: Option<bool>,
    /// Name of the variable.
    pub name: String,
    /// Sensitive variables are never shown in the UI or API.
    /// They may appear in Terraform logs if your configuration is designed to output them.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// Value of the variable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<WorkspaceEnvironmentVariablesValueFrom>,
}

/// Source for the variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceEnvironmentVariablesValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<WorkspaceEnvironmentVariablesValueFromConfigMapKeyRef>,
    /// Selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<WorkspaceEnvironmentVariablesValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceEnvironmentVariablesValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceEnvironmentVariablesValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Notifications allow you to send messages to other applications based on run and workspace events.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/notifications
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceNotifications {
    /// The list of email addresses that will receive notification emails.
    /// It is only available for Terraform Enterprise users. It is not available in Terraform Cloud.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emailAddresses")]
    pub email_addresses: Option<Vec<String>>,
    /// The list of users belonging to the organization that will receive notification emails.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emailUsers")]
    pub email_users: Option<Vec<String>>,
    /// Whether the notification configuration should be enabled or not.
    /// Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Notification name.
    pub name: String,
    /// The token of the notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The list of run events that will trigger notifications.
    /// Trigger represents the different TFC notifications that can be sent as a run's progress transitions between different states.
    /// There are two categories of triggers:
    ///   - Health Events: `assessment:check_failure`, `assessment:drifted`, `assessment:failed`.
    ///   - Run Events: `run:applying`, `run:completed`, `run:created`, `run:errored`, `run:needs_attention`, `run:planning`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<String>>,
    /// The type of the notification.
    /// Must be one of the following values: `email`, `generic`, `microsoft-teams`, `slack`.
    #[serde(rename = "type")]
    pub r#type: WorkspaceNotificationsType,
    /// The URL of the notification.
    /// Must match pattern: `^https?://.*`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Notifications allow you to send messages to other applications based on run and workspace events.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/notifications
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WorkspaceNotificationsType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "microsoft-teams")]
    MicrosoftTeams,
    #[serde(rename = "slack")]
    Slack,
}

/// Projects let you organize your workspaces into groups.
/// Default: default organization project.
/// More information:
///   - https://developer.hashicorp.com/terraform/tutorials/cloud/projects
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceProject {
    /// Project ID.
    /// Must match pattern: `^prj-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Project name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Remote state access between workspaces.
/// By default, new workspaces in Terraform Cloud do not allow other workspaces to access their state.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/state#accessing-state-from-other-workspaces
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceRemoteStateSharing {
    /// Allow access to the state for all workspaces within the same organization.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allWorkspaces")]
    pub all_workspaces: Option<bool>,
    /// Allow access to the state for specific workspaces within the same organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<WorkspaceRemoteStateSharingWorkspaces>>,
}

/// ConsumerWorkspace allows access to the state for specific workspaces within the same organization.
/// Only one of the fields `ID` or `Name` is allowed.
/// At least one of the fields `ID` or `Name` is mandatory.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/state#remote-state-access-controls
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceRemoteStateSharingWorkspaces {
    /// Consumer Workspace ID.
    /// Must match pattern: `^ws-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Consumer Workspace name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Run tasks allow Terraform Cloud to interact with external systems at specific points in the Terraform Cloud run lifecycle.
/// Only one of the fields `ID` or `Name` is allowed.
/// At least one of the fields `ID` or `Name` is mandatory.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/run-tasks
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceRunTasks {
    /// Run Task Enforcement Level. Can be one of `advisory` or `mandatory`. Default: `advisory`.
    /// Must be one of the following values: `advisory`, `mandatory`
    /// Default: `advisory`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enforcementLevel")]
    pub enforcement_level: Option<String>,
    /// Run Task ID.
    /// Must match pattern: `^task-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Run Task Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Run Task Stage.
    /// Must be one of the following values: `pre_apply`, `pre_plan`, `post_plan`.
    /// Default: `post_plan`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// RunTrigger allows you to connect this workspace to one or more source workspaces.
/// These connections allow runs to queue automatically in this workspace on successful apply of runs in any of the source workspaces.
/// Only one of the fields `ID` or `Name` is allowed.
/// At least one of the fields `ID` or `Name` is mandatory.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/run-triggers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceRunTriggers {
    /// Source Workspace ID.
    /// Must match pattern: `^ws-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Source Workspace Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SSH key used to clone Terraform modules.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/ssh-keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceSshKey {
    /// SSH key ID.
    /// Must match pattern: `^sshkey-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// SSH key name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Terraform Cloud workspaces can only be accessed by users with the correct permissions.
/// You can manage permissions for a workspace on a per-team basis.
/// When a workspace is created, only the owners team and teams with the "manage workspaces" permission can access it,
/// with full admin permissions. These teams' access can't be removed from a workspace.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/settings/access
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTeamAccess {
    /// There are two ways to choose which permissions a given team has on a workspace: fixed permission sets, and custom permissions.
    /// Must be one of the following values: `admin`, `custom`, `plan`, `read`, `write`.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/users-teams-organizations/permissions#workspace-permissions
    pub access: String,
    /// Custom permissions let you assign specific, finer-grained permissions to a team than the broader fixed permission sets provide.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/users-teams-organizations/permissions#custom-workspace-permissions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<WorkspaceTeamAccessCustom>,
    /// Team to grant access.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/users-teams-organizations/teams
    pub team: WorkspaceTeamAccessTeam,
}

/// Custom permissions let you assign specific, finer-grained permissions to a team than the broader fixed permission sets provide.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/users-teams-organizations/permissions#custom-workspace-permissions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTeamAccessCustom {
    /// Manage Workspace Run Tasks.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runTasks")]
    pub run_tasks: Option<bool>,
    /// Run access.
    /// Must be one of the following values: `apply`, `plan`, `read`.
    /// Default: `read`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<String>,
    /// Download Sentinel mocks.
    /// Must be one of the following values: `none`, `read`.
    /// Default: `none`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sentinel: Option<String>,
    /// State access.
    /// Must be one of the following values: `none`, `read`, `read-outputs`, `write`.
    /// Default: `none`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stateVersions")]
    pub state_versions: Option<String>,
    /// Variable access.
    /// Must be one of the following values: `none`, `read`, `write`.
    /// Default: `none`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<String>,
    /// Lock/unlock workspace.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workspaceLocking")]
    pub workspace_locking: Option<bool>,
}

/// Team to grant access.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/users-teams-organizations/teams
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTeamAccessTeam {
    /// Team ID.
    /// Must match pattern: `^team-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Team name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Variables let you customize configurations, modify Terraform's behavior, and store information like provider credentials.
/// More information:
///   - https://developer.hashicorp.com/terraform/cloud-docs/workspaces/variables
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTerraformVariables {
    /// Description of the variable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parse this field as HashiCorp Configuration Language (HCL). This allows you to interpolate values at runtime.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hcl: Option<bool>,
    /// Name of the variable.
    pub name: String,
    /// Sensitive variables are never shown in the UI or API.
    /// They may appear in Terraform logs if your configuration is designed to output them.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// Value of the variable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<WorkspaceTerraformVariablesValueFrom>,
}

/// Source for the variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTerraformVariablesValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<WorkspaceTerraformVariablesValueFromConfigMapKeyRef>,
    /// Selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<WorkspaceTerraformVariablesValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTerraformVariablesValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTerraformVariablesValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// API Token to be used for API calls.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceToken {
    /// Selects a key of a secret in the workspace's namespace
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: WorkspaceTokenSecretKeyRef,
}

/// Selects a key of a secret in the workspace's namespace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceTokenSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Settings for the workspace's VCS repository, enabling the UI/VCS-driven run workflow.
/// Omit this argument to utilize the CLI-driven and API-driven workflows, where runs are not driven by webhooks on your VCS provider.
/// More information:
///   - https://www.terraform.io/cloud-docs/run/ui
///   - https://www.terraform.io/cloud-docs/vcs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceVersionControl {
    /// The repository branch that Run will execute from. This defaults to the repository's default branch (e.g. main).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The VCS Connection (OAuth Connection + Token) to use.
    /// Must match pattern: `^ot-[a-zA-Z0-9]+$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oAuthTokenID")]
    pub o_auth_token_id: Option<String>,
    /// A reference to your VCS repository in the format `<organization>/<repository>` where `<organization>` and `<repository>` refer to the organization and repository in your VCS provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// Whether this workspace allows automatic speculative plans on PR.
    /// Default: `true`.
    /// More information:
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/run/ui#speculative-plans-on-pull-requests
    ///   - https://developer.hashicorp.com/terraform/cloud-docs/run/remote-operations#speculative-plans
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "speculativePlans")]
    pub speculative_plans: Option<bool>,
}

/// WorkspaceStatus defines the observed state of Workspace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceStatus {
    /// Real world state generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Run status of plan-only/speculative plan that was triggered manually.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<WorkspaceStatusPlan>,
    /// Workspace Runs status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runStatus")]
    pub run_status: Option<WorkspaceStatusRunStatus>,
    /// Workspace Terraform version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terraformVersion")]
    pub terraform_version: Option<String>,
    /// Workspace last update timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateAt")]
    pub update_at: Option<i64>,
    /// Workspace variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<WorkspaceStatusVariables>>,
    /// Workspace ID that is managed by the controller.
    #[serde(rename = "workspaceID")]
    pub workspace_id: String,
}

/// Run status of plan-only/speculative plan that was triggered manually.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceStatusPlan {
    /// Latest plan-only/speculative plan Terraform Cloud run ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Latest plan-only/speculative plan Terraform Cloud run status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The version of Terraform to use for this run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terraformVersion")]
    pub terraform_version: Option<String>,
}

/// Workspace Runs status.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceStatusRunStatus {
    /// The configuration version of this run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationVersion")]
    pub configuration_version: Option<String>,
    /// Current(both active and finished) Terraform Cloud run ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Run ID of the latest run that could update the outputs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputRunID")]
    pub output_run_id: Option<String>,
    /// Current(both active and finished) Terraform Cloud run status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkspaceStatusVariables {
    /// Category of the variable.
    pub category: String,
    /// ID of the variable.
    pub id: String,
    /// Name of the variable.
    pub name: String,
    /// ValueID is a hash of the variable on the CRD end.
    #[serde(rename = "valueID")]
    pub value_id: String,
    /// VersionID is a hash of the variable on the TFC end.
    #[serde(rename = "versionID")]
    pub version_id: String,
}

