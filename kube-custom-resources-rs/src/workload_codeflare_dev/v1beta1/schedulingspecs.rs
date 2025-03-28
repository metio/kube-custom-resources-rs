// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/project-codeflare/codeflare-operator/workload.codeflare.dev/v1beta1/schedulingspecs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "workload.codeflare.dev", version = "v1beta1", kind = "SchedulingSpec", plural = "schedulingspecs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SchedulingSpecSpec {
    /// Wall clock duration time of appwrapper in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dispatchDuration")]
    pub dispatch_duration: Option<SchedulingSpecDispatchDuration>,
    /// Expected number of pods in running and/or completed state. Requeuing is triggered when the number of running/completed pods is not equal to this value. When not specified, requeuing is disabled and no check is performed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minAvailable")]
    pub min_available: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// Specification of the requeuing strategy based on waiting time. Values in this field control how often the pod check should happen, and if requeuing has reached its maximum number of times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requeuing: Option<SchedulingSpecRequeuing>,
}

/// Wall clock duration time of appwrapper in seconds.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SchedulingSpecDispatchDuration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrun: Option<bool>,
}

/// Specification of the requeuing strategy based on waiting time. Values in this field control how often the pod check should happen, and if requeuing has reached its maximum number of times.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SchedulingSpecRequeuing {
    /// Growth strategy to increase the waiting time between requeuing checks. The values available are 'exponential', 'linear', or 'none'. For example, 'exponential' growth would double the 'timeInSeconds' value every time a requeuing event is triggered. If the string value is misspelled or not one of the possible options, the growth behavior is defaulted to 'none'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "growthType")]
    pub growth_type: Option<String>,
    /// Value to keep track of the initial wait time. Users cannot set this as it is taken from 'timeInSeconds'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialTimeInSeconds")]
    pub initial_time_in_seconds: Option<i64>,
    /// Maximum number of requeuing events allowed. Once this value is reached (e.g., 'numRequeuings = maxNumRequeuings', no more requeuing checks are performed and the generic items are stopped and removed from the cluster (AppWrapper remains deployed).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxNumRequeuings")]
    pub max_num_requeuings: Option<i64>,
    /// Maximum waiting time for requeuing checks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTimeInSeconds")]
    pub max_time_in_seconds: Option<i64>,
    /// Field to keep track of how many times a requeuing event has been triggered.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numRequeuings")]
    pub num_requeuings: Option<i64>,
    /// Initial waiting time before requeuing conditions are checked. This value is specified by the user, but it may grow as requeuing events happen.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeInSeconds")]
    pub time_in_seconds: Option<i64>,
}

