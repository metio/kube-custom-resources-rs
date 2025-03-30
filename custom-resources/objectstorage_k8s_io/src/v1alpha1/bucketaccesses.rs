// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/container-object-storage-interface/objectstorage.k8s.io/v1alpha1/bucketaccesses.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "objectstorage.k8s.io", version = "v1alpha1", kind = "BucketAccess", plural = "bucketaccesses")]
#[kube(namespaced)]
#[kube(status = "BucketAccessStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BucketAccessSpec {
    /// BucketAccessClassName is the name of the BucketAccessClass
    #[serde(rename = "bucketAccessClassName")]
    pub bucket_access_class_name: String,
    /// BucketClaimName is the name of the BucketClaim.
    #[serde(rename = "bucketClaimName")]
    pub bucket_claim_name: String,
    /// CredentialsSecretName is the name of the secret that COSI should populate
    /// with the credentials. If a secret by this name already exists, then it is
    /// assumed that credentials have already been generated. It is not overridden.
    /// This secret is deleted when the BucketAccess is delted.
    #[serde(rename = "credentialsSecretName")]
    pub credentials_secret_name: String,
    /// Protocol is the name of the Protocol
    /// that this access credential is supposed to support
    /// If left empty, it will choose the protocol supported
    /// by the bucket. If the bucket supports multiple protocols,
    /// the end protocol is determined by the driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// ServiceAccountName is the name of the serviceAccount that COSI will map
    /// to the OSP service account when IAM styled authentication is specified
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketAccessStatus {
    /// AccessGranted indicates the successful grant of privileges to access the bucket
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessGranted")]
    pub access_granted: Option<bool>,
    /// AccountID is the unique ID for the account in the OSP. It will be populated
    /// by the COSI sidecar once access has been successfully granted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountID")]
    pub account_id: Option<String>,
}

