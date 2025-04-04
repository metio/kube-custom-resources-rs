// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/gitlab-org/gl-openshift/gitlab-runner-operator/apps.gitlab.com/v1beta2/runners.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Specification of the desired behavior of a GitLab Runner instance
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.gitlab.com", version = "v1beta2", kind = "Runner", plural = "runners")]
#[kube(namespaced)]
#[kube(status = "RunnerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RunnerSpec {
    /// options used to setup Azure blob
    /// storage as GitLab Runner Cache
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<RunnerAzure>,
    /// The name of the default image to use to run
    /// build jobs, when none is specified
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "buildImage")]
    pub build_image: Option<String>,
    /// Name of tls secret containing the custom certificate
    /// authority (CA) certificates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<String>,
    /// Path defines the Runner Cache path
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cachePath")]
    pub cache_path: Option<String>,
    /// Enable sharing of cache between Runners
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheShared")]
    pub cache_shared: Option<bool>,
    /// Type of cache used for Runner artifacts
    /// Options are: gcs, s3, azure
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheType")]
    pub cache_type: Option<String>,
    /// If specified, overrides the default URL used to clone or fetch the Git ref
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneURL")]
    pub clone_url: Option<String>,
    /// Option to limit the number of jobs globally that can run concurrently.
    /// The operator sets this to 10, if not specified
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent: Option<i32>,
    /// allow user to provide configmap name
    /// containing the user provided config.toml
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// The maximum duration a TLS keepalive connection to the GitLab server should remain open before reconnecting. The default value is `15m` for 15 minutes. If set to `0` or lower, the connection persists as long as possible.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionMaxAge")]
    pub connection_max_age: Option<String>,
    /// Accepts configmap name. Provides user mechanism to inject environment
    /// variables in the GitLab Runner pod via the key value pairs in the ConfigMap
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<String>,
    /// options used to setup GCS (Google
    /// Container Storage) as GitLab Runner Cache
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<RunnerGcs>,
    /// The fully qualified domain name for the GitLab instance.
    /// For example, https://gitlab.example.com
    #[serde(rename = "gitlabUrl")]
    pub gitlab_url: String,
    /// If specified, overrides the default GitLab Runner helper image
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "helperImage")]
    pub helper_image: Option<String>,
    /// ImagePullPolicy sets the Image pull policy.
    /// One of Always, Never, IfNotPresent.
    /// Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
    /// More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Option to define the number of seconds between checks for new jobs.
    /// This is set to a default of 30s by operator if not set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    /// Option to set the metrics listen address for the runner.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenAddr")]
    pub listen_addr: Option<String>,
    /// Specify whether the runner should be locked to a specific project. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Specifies the log format. Options are `runner`, `text`, and `json`. The default value is `runner`, which contains ANSI escape codes for coloring.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logFormat")]
    pub log_format: Option<String>,
    /// Option to set the log level for the runner.
    /// Valid values are "debug", "info", "warn", "error", "fatal", "panic"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<String>,
    /// If specified, overrides the namespace where job pods are created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSpec")]
    pub pod_spec: Option<Vec<RunnerPodSpec>>,
    /// Specify whether the runner should only run protected branches. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    /// Specify if jobs without tags should be run.
    /// If not specified, runner will default to true if no tags were specified.
    /// In other case it will default to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runUntagged")]
    pub run_untagged: Option<bool>,
    /// If specified, overrides the default GitLab Runner image. Default is the Runner image the operator was bundled with.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runnerImage")]
    pub runner_image: Option<String>,
    /// options used to setup S3
    /// object store as GitLab Runner Cache
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<RunnerS3>,
    /// Enables tracking of all system level errors to Sentry.
    /// If not specified, error tracking with Sentry will be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sentryDsn")]
    pub sentry_dsn: Option<String>,
    /// allow user to override service account
    /// used by GitLab Runner
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serviceaccount: Option<String>,
    /// Number of seconds until the forceful shutdown operation times out and exits the process. The default value is `30`. If set to `0` or lower, the default value is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shutdownTimeout")]
    pub shutdown_timeout: Option<i32>,
    /// List of comma separated tags to be applied to the runner
    /// More info: https://docs.gitlab.com/ee/ci/runners/#use-tags-to-limit-the-number-of-jobs-using-the-runner
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Name of secret containing the 'runner-registration-token' key used to register the runner
    pub token: String,
}

/// options used to setup Azure blob
/// storage as GitLab Runner Cache
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RunnerAzure {
    /// Name of the Azure container in which the cache will be stored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Credentials secret contains 'accountName' and 'privateKey'
    /// used to authenticate against Azure blob storage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// The domain name of the Azure blob storage
    /// e.g. blob.core.windows.net
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageDomain")]
    pub storage_domain: Option<String>,
}

/// options used to setup GCS (Google
/// Container Storage) as GitLab Runner Cache
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RunnerGcs {
    /// Name of the bucket in which the cache will be stored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// contains the GCS 'access-id' and 'private-key'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// Takes GCS credentials file, 'keys.json'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsFile")]
    pub credentials_file: Option<String>,
}

/// KubernetesPodSpec represents the structure expected when adding a custom PodSpec to configure
/// the Pod running the GitLab Runner Manager
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RunnerPodSpec {
    /// Name is the name given to the custom Pod Spec
    pub name: String,
    /// A JSON or YAML format string that describes the changes which must be applied
    /// to the final PodSpec object before it is generated.
    /// You cannot set the patch_path and patch in the same pod_spec configuration, otherwise an error occurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
    /// Path to the file that defines the changes to apply to the final PodSpec object before it is generated.
    /// The file must be a JSON or YAML file.
    /// You cannot set the patch_path and patch in the same pod_spec configuration, otherwise an error occurs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "patchFile")]
    pub patch_file: Option<String>,
    /// The strategy the runner uses to apply the specified changes to the PodSpec object generated by GitLab Runner.
    /// The accepted values are merge, json, and strategic (default value).
    #[serde(rename = "patchType")]
    pub patch_type: String,
}

/// options used to setup S3
/// object store as GitLab Runner Cache
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RunnerS3 {
    /// Name of the bucket in which the cache will be stored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// Name of the secret containing the
    /// 'accesskey' and 'secretkey' used to access the object storage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// Use insecure connections or HTTP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// Name of the S3 region in use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// Most recently observed status of the GitLab Runner.
/// It is read-only to the user
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RunnerStatus {
    /// Additional information of GitLab Runner registration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Reports status of the GitLab Runner instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Reports status of GitLab Runner registration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<String>,
}

