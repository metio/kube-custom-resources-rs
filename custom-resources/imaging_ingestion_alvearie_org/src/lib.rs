/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# imaging_ingestion_alvearie_org

Custom resources in this crate belong to the `imaging-ingestion.alvearie.org` group. The following versions and custom resources are available:

## imaging-ingestion.alvearie.org/v1alpha1
- `DicomEventBridge`
- `DicomEventDrivenIngestion`
- `DicomInstanceBinding`
- `DicomStudyBinding`
- `DicomwebIngestionService`
- `DimseIngestionService`
- `DimseProxy`
*/
pub mod v1alpha1;
