// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/acmpca-controller/acmpca.services.k8s.aws/v1alpha1/certificates.yaml --derive=Default --derive=PartialEq
// kopium version: 0.18.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// CertificateSpec defines the desired state of Certificate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "acmpca.services.k8s.aws", version = "v1alpha1", kind = "Certificate", plural = "certificates")]
#[kube(namespaced)]
#[kube(status = "CertificateStatus")]
#[kube(schema = "disabled")]
pub struct CertificateSpec {
    /// Specifies X.509 certificate information to be included in the issued certificate.
    /// An APIPassthrough or APICSRPassthrough template variant must be selected,
    /// or else this parameter is ignored. For more information about using these
    /// templates, see Understanding Certificate Templates (https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html).
    /// 
    /// 
    /// If conflicting or duplicate certificate information is supplied during certificate
    /// issuance, Amazon Web Services Private CA applies order of operation rules
    /// (https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html#template-order-of-operations)
    /// to determine what information is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiPassthrough")]
    pub api_passthrough: Option<CertificateApiPassthrough>,
    /// The Amazon Resource Name (ARN) that was returned when you called CreateCertificateAuthority
    /// (https://docs.aws.amazon.com/privateca/latest/APIReference/API_CreateCertificateAuthority.html).
    /// This must be of the form:
    /// 
    /// 
    /// arn:aws:acm-pca:region:account:certificate-authority/12345678-1234-1234-1234-123456789012
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateAuthorityARN")]
    pub certificate_authority_arn: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateAuthorityRef")]
    pub certificate_authority_ref: Option<CertificateCertificateAuthorityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateSigningRequest")]
    pub certificate_signing_request: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateSigningRequestRef")]
    pub certificate_signing_request_ref: Option<CertificateCertificateSigningRequestRef>,
    /// The name of the algorithm that will be used to sign the certificate to be
    /// issued.
    /// 
    /// 
    /// This parameter should not be confused with the SigningAlgorithm parameter
    /// used to sign a CSR in the CreateCertificateAuthority action.
    /// 
    /// 
    /// The specified signing algorithm family (RSA or ECDSA) must match the algorithm
    /// family of the CA's secret key.
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
    /// Specifies a custom configuration template to use when issuing a certificate.
    /// If this parameter is not provided, Amazon Web Services Private CA defaults
    /// to the EndEntityCertificate/V1 template. For CA certificates, you should
    /// choose the shortest path length that meets your needs. The path length is
    /// indicated by the PathLenN portion of the ARN, where N is the CA depth (https://docs.aws.amazon.com/privateca/latest/userguide/PcaTerms.html#terms-cadepth).
    /// 
    /// 
    /// Note: The CA depth configured on a subordinate CA certificate must not exceed
    /// the limit set by its parents in the CA hierarchy.
    /// 
    /// 
    /// For a list of TemplateArn values supported by Amazon Web Services Private
    /// CA, see Understanding Certificate Templates (https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateARN")]
    pub template_arn: Option<String>,
    /// Information describing the end of the validity period of the certificate.
    /// This parameter sets the “Not After” date for the certificate.
    /// 
    /// 
    /// Certificate validity is the period of time during which a certificate is
    /// valid. Validity can be expressed as an explicit date and time when the certificate
    /// expires, or as a span of time after issuance, stated in days, months, or
    /// years. For more information, see Validity (https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.5)
    /// in RFC 5280.
    /// 
    /// 
    /// This value is unaffected when ValidityNotBefore is also specified. For example,
    /// if Validity is set to 20 days in the future, the certificate will expire
    /// 20 days from issuance time regardless of the ValidityNotBefore value.
    /// 
    /// 
    /// The end of the validity period configured on a certificate must not exceed
    /// the limit set on its parents in the CA hierarchy.
    pub validity: CertificateValidity,
    /// Information describing the start of the validity period of the certificate.
    /// This parameter sets the “Not Before" date for the certificate.
    /// 
    /// 
    /// By default, when issuing a certificate, Amazon Web Services Private CA sets
    /// the "Not Before" date to the issuance time minus 60 minutes. This compensates
    /// for clock inconsistencies across computer systems. The ValidityNotBefore
    /// parameter can be used to customize the “Not Before” value.
    /// 
    /// 
    /// Unlike the Validity parameter, the ValidityNotBefore parameter is optional.
    /// 
    /// 
    /// The ValidityNotBefore value is expressed as an explicit date and time, using
    /// the Validity type value ABSOLUTE. For more information, see Validity (https://docs.aws.amazon.com/privateca/latest/APIReference/API_Validity.html)
    /// in this API reference and Validity (https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.5)
    /// in RFC 5280.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validityNotBefore")]
    pub validity_not_before: Option<CertificateValidityNotBefore>,
}

/// Specifies X.509 certificate information to be included in the issued certificate.
/// An APIPassthrough or APICSRPassthrough template variant must be selected,
/// or else this parameter is ignored. For more information about using these
/// templates, see Understanding Certificate Templates (https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html).
/// 
/// 
/// If conflicting or duplicate certificate information is supplied during certificate
/// issuance, Amazon Web Services Private CA applies order of operation rules
/// (https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html#template-order-of-operations)
/// to determine what information is used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthrough {
    /// Contains X.509 extension information for a certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<CertificateApiPassthroughExtensions>,
    /// Contains information about the certificate subject. The Subject field in
    /// the certificate identifies the entity that owns or controls the public key
    /// in the certificate. The entity can be a user, computer, device, or service.
    /// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
    /// of relative distinguished names (RDNs). The RDNs are separated by commas
    /// in the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<CertificateApiPassthroughSubject>,
}

/// Contains X.509 extension information for a certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificatePolicies")]
    pub certificate_policies: Option<Vec<CertificateApiPassthroughExtensionsCertificatePolicies>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customExtensions")]
    pub custom_extensions: Option<Vec<CertificateApiPassthroughExtensionsCustomExtensions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extendedKeyUsage")]
    pub extended_key_usage: Option<Vec<CertificateApiPassthroughExtensionsExtendedKeyUsage>>,
    /// Defines one or more purposes for which the key contained in the certificate
    /// can be used. Default value for each option is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyUsage")]
    pub key_usage: Option<CertificateApiPassthroughExtensionsKeyUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAlternativeNames")]
    pub subject_alternative_names: Option<Vec<CertificateApiPassthroughExtensionsSubjectAlternativeNames>>,
}

/// Defines the X.509 CertificatePolicies extension.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsCertificatePolicies {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certPolicyID")]
    pub cert_policy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyQualifiers")]
    pub policy_qualifiers: Option<Vec<CertificateApiPassthroughExtensionsCertificatePoliciesPolicyQualifiers>>,
}

/// Modifies the CertPolicyId of a PolicyInformation object with a qualifier.
/// Amazon Web Services Private CA supports the certification practice statement
/// (CPS) qualifier.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsCertificatePoliciesPolicyQualifiers {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyQualifierID")]
    pub policy_qualifier_id: Option<String>,
    /// Defines a PolicyInformation qualifier. Amazon Web Services Private CA supports
    /// the certification practice statement (CPS) qualifier (https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.4)
    /// defined in RFC 5280.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<CertificateApiPassthroughExtensionsCertificatePoliciesPolicyQualifiersQualifier>,
}

/// Defines a PolicyInformation qualifier. Amazon Web Services Private CA supports
/// the certification practice statement (CPS) qualifier (https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.4)
/// defined in RFC 5280.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsCertificatePoliciesPolicyQualifiersQualifier {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpsURI")]
    pub cps_uri: Option<String>,
}

/// Specifies the X.509 extension information for a certificate.
/// 
/// 
/// Extensions present in CustomExtensions follow the ApiPassthrough template
/// rules (https://docs.aws.amazon.com/privateca/latest/userguide/UsingTemplates.html#template-order-of-operations).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsCustomExtensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectIdentifier")]
    pub object_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Specifies additional purposes for which the certified public key may be used
/// other than basic purposes indicated in the KeyUsage extension.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsExtendedKeyUsage {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extendedKeyUsageObjectIdentifier")]
    pub extended_key_usage_object_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extendedKeyUsageType")]
    pub extended_key_usage_type: Option<String>,
}

/// Defines one or more purposes for which the key contained in the certificate
/// can be used. Default value for each option is false.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsKeyUsage {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crlSign")]
    pub crl_sign: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEncipherment")]
    pub data_encipherment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "decipherOnly")]
    pub decipher_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "digitalSignature")]
    pub digital_signature: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encipherOnly")]
    pub encipher_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyAgreement")]
    pub key_agreement: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyCertSign")]
    pub key_cert_sign: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyEncipherment")]
    pub key_encipherment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonRepudiation")]
    pub non_repudiation: Option<bool>,
}

/// Describes an ASN.1 X.400 GeneralName as defined in RFC 5280 (https://datatracker.ietf.org/doc/html/rfc5280).
/// Only one of the following naming options should be provided. Providing more
/// than one option results in an InvalidArgsException error.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsSubjectAlternativeNames {
    /// Contains information about the certificate subject. The Subject field in
    /// the certificate identifies the entity that owns or controls the public key
    /// in the certificate. The entity can be a user, computer, device, or service.
    /// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
    /// of relative distinguished names (RDNs). The RDNs are separated by commas
    /// in the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "directoryName")]
    pub directory_name: Option<CertificateApiPassthroughExtensionsSubjectAlternativeNamesDirectoryName>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    /// Describes an Electronic Data Interchange (EDI) entity as described in as
    /// defined in Subject Alternative Name (https://datatracker.ietf.org/doc/html/rfc5280)
    /// in RFC 5280.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ediPartyName")]
    pub edi_party_name: Option<CertificateApiPassthroughExtensionsSubjectAlternativeNamesEdiPartyName>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddress")]
    pub ip_address: Option<String>,
    /// Defines a custom ASN.1 X.400 GeneralName using an object identifier (OID)
    /// and value. The OID must satisfy the regular expression shown below. For more
    /// information, see NIST's definition of Object Identifier (OID) (https://csrc.nist.gov/glossary/term/Object_Identifier).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherName")]
    pub other_name: Option<CertificateApiPassthroughExtensionsSubjectAlternativeNamesOtherName>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registeredID")]
    pub registered_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rfc822Name")]
    pub rfc822_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uniformResourceIdentifier")]
    pub uniform_resource_identifier: Option<String>,
}

/// Contains information about the certificate subject. The Subject field in
/// the certificate identifies the entity that owns or controls the public key
/// in the certificate. The entity can be a user, computer, device, or service.
/// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
/// of relative distinguished names (RDNs). The RDNs are separated by commas
/// in the certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsSubjectAlternativeNamesDirectoryName {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customAttributes")]
    pub custom_attributes: Option<Vec<CertificateApiPassthroughExtensionsSubjectAlternativeNamesDirectoryNameCustomAttributes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "distinguishedNameQualifier")]
    pub distinguished_name_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generationQualifier")]
    pub generation_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "givenName")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "organizationalUnit")]
    pub organizational_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serialNumber")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Defines the X.500 relative distinguished name (RDN).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsSubjectAlternativeNamesDirectoryNameCustomAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectIdentifier")]
    pub object_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Describes an Electronic Data Interchange (EDI) entity as described in as
/// defined in Subject Alternative Name (https://datatracker.ietf.org/doc/html/rfc5280)
/// in RFC 5280.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsSubjectAlternativeNamesEdiPartyName {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameAssigner")]
    pub name_assigner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partyName")]
    pub party_name: Option<String>,
}

/// Defines a custom ASN.1 X.400 GeneralName using an object identifier (OID)
/// and value. The OID must satisfy the regular expression shown below. For more
/// information, see NIST's definition of Object Identifier (OID) (https://csrc.nist.gov/glossary/term/Object_Identifier).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughExtensionsSubjectAlternativeNamesOtherName {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typeID")]
    pub type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Contains information about the certificate subject. The Subject field in
/// the certificate identifies the entity that owns or controls the public key
/// in the certificate. The entity can be a user, computer, device, or service.
/// The Subject must contain an X.500 distinguished name (DN). A DN is a sequence
/// of relative distinguished names (RDNs). The RDNs are separated by commas
/// in the certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughSubject {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customAttributes")]
    pub custom_attributes: Option<Vec<CertificateApiPassthroughSubjectCustomAttributes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "distinguishedNameQualifier")]
    pub distinguished_name_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generationQualifier")]
    pub generation_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "givenName")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "organizationalUnit")]
    pub organizational_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serialNumber")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Defines the X.500 relative distinguished name (RDN).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateApiPassthroughSubjectCustomAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectIdentifier")]
    pub object_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateCertificateAuthorityRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<CertificateCertificateAuthorityRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateCertificateAuthorityRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateCertificateSigningRequestRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<CertificateCertificateSigningRequestRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateCertificateSigningRequestRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Information describing the end of the validity period of the certificate.
/// This parameter sets the “Not After” date for the certificate.
/// 
/// 
/// Certificate validity is the period of time during which a certificate is
/// valid. Validity can be expressed as an explicit date and time when the certificate
/// expires, or as a span of time after issuance, stated in days, months, or
/// years. For more information, see Validity (https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.5)
/// in RFC 5280.
/// 
/// 
/// This value is unaffected when ValidityNotBefore is also specified. For example,
/// if Validity is set to 20 days in the future, the certificate will expire
/// 20 days from issuance time regardless of the ValidityNotBefore value.
/// 
/// 
/// The end of the validity period configured on a certificate must not exceed
/// the limit set on its parents in the CA hierarchy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateValidity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// Information describing the start of the validity period of the certificate.
/// This parameter sets the “Not Before" date for the certificate.
/// 
/// 
/// By default, when issuing a certificate, Amazon Web Services Private CA sets
/// the "Not Before" date to the issuance time minus 60 minutes. This compensates
/// for clock inconsistencies across computer systems. The ValidityNotBefore
/// parameter can be used to customize the “Not Before” value.
/// 
/// 
/// Unlike the Validity parameter, the ValidityNotBefore parameter is optional.
/// 
/// 
/// The ValidityNotBefore value is expressed as an explicit date and time, using
/// the Validity type value ABSOLUTE. For more information, see Validity (https://docs.aws.amazon.com/privateca/latest/APIReference/API_Validity.html)
/// in this API reference and Validity (https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.5)
/// in RFC 5280.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateValidityNotBefore {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// CertificateStatus defines the observed state of Certificate
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<CertificateStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CertificateStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

