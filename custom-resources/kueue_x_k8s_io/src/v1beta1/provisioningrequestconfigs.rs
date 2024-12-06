// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/provisioningrequestconfigs.yaml
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
    /// retryStrategy defines strategy for retrying ProvisioningRequest.
    /// If null, then the default configuration is applied with the following parameter values:
    /// backoffLimitCount:  3
    /// backoffBaseSeconds: 60 - 1 min
    /// backoffMaxSeconds:  1800 - 30 mins
    /// 
    /// To switch off retry mechanism
    /// set retryStrategy.backoffLimitCount to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryStrategy")]
    pub retry_strategy: Option<ProvisioningRequestConfigRetryStrategy>,
}

/// retryStrategy defines strategy for retrying ProvisioningRequest.
/// If null, then the default configuration is applied with the following parameter values:
/// backoffLimitCount:  3
/// backoffBaseSeconds: 60 - 1 min
/// backoffMaxSeconds:  1800 - 30 mins
/// 
/// To switch off retry mechanism
/// set retryStrategy.backoffLimitCount to 0.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProvisioningRequestConfigRetryStrategy {
    /// BackoffBaseSeconds defines the base for the exponential backoff for
    /// re-queuing an evicted workload.
    /// 
    /// Defaults to 60.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backoffBaseSeconds")]
    pub backoff_base_seconds: Option<i32>,
    /// BackoffLimitCount defines the maximum number of re-queuing retries.
    /// Once the number is reached, the workload is deactivated (`.spec.activate`=`false`).
    /// 
    /// Every backoff duration is about "b*2^(n-1)+Rand" where:
    /// - "b" represents the base set by "BackoffBaseSeconds" parameter,
    /// - "n" represents the "workloadStatus.requeueState.count",
    /// - "Rand" represents the random jitter.
    /// During this time, the workload is taken as an inadmissible and
    /// other workloads will have a chance to be admitted.
    /// By default, the consecutive requeue delays are around: (60s, 120s, 240s, ...).
    /// 
    /// Defaults to 3.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backoffLimitCount")]
    pub backoff_limit_count: Option<i32>,
    /// BackoffMaxSeconds defines the maximum backoff time to re-queue an evicted workload.
    /// 
    /// Defaults to 1800.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backoffMaxSeconds")]
    pub backoff_max_seconds: Option<i32>,
}

