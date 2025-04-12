<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# Kubernetes Custom Resource Bindings for Rust [![Chat](https://img.shields.io/badge/matrix-%23talk.metio:matrix.org-brightgreen.svg?style=social&label=Matrix)](https://matrix.to/#/#talk.metio:matrix.org)

This repository contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) generated with [kopium](https://github.com/kube-rs/kopium).

Feel free to add your own CRD to the [catalog](https://github.com/metio/kube-custom-resources-rs/blob/main/code-generator/src/catalog.rs)!

## Structure

Originally, this repository contained a single crate for all generated custom resources. However, crates.io has a size limit of 10MiB which we reached and thus this repository was split into multiple smaller crates, one for each custom resource group. Since there are no namespaces on crates.io, we prefix every crate in this repository with `kcr_` (kube-custom-resource) and hope that no name conflict will happen in the future.

## Custom Resources

Each group of a Kubernetes custom resource has a corresponding crate in this repository. The group of a custom resource can be seen in the `apiVersion` field of a resource, e.g.:

```yaml
apiVersion: chaos-mesh.org/v1alpha1
kind: PodNetworkChaos
metadata:
  ...
```

In the above example, `chaos-mesh.org` is the group and `v1alpha1` is the version. Since Cargo imposes certain rules on how crates can be named, `.` and `-` are mapped to `_`. Therefore, the crate that contains the custom resource from the example above is called `chaos_mesh_org` and can be enabled like this (using the previously mentioned `kcr_` prefix):

```toml
[dependencies]
kcr_chaos_mesh_org = "<version>"
```

Replace `<version>` with the latest available release of that crate. Each version within a group has a corresponding module in the associated crate, e.g. there is a module called `v1alpha1` in the crate `kcr_chaos_mesh_org`.

## Versioning

This crate uses a calendar based versioning scheme because resources in Kubernetes are versioned themselves.

Updates to all CRDs are fetched [automatically](https://github.com/metio/kube-custom-resources-rs/blob/main/.github/workflows/update-crds.yml) and released every sunday if any changes were detected.

## Available Groups

The following groups are available:

- [about.k8s.io](https://crates.io/crates/kcr_about_k8s_io)
- [acid.zalan.do](https://crates.io/crates/kcr_acid_zalan_do)
- [acme.cert-manager.io](https://crates.io/crates/kcr_acme_cert_manager_io)
- [acmpca.services.k8s.aws](https://crates.io/crates/kcr_acmpca_services_k8s_aws)
- [actions.github.com](https://crates.io/crates/kcr_actions_github_com)
- [actions.summerwind.dev](https://crates.io/crates/kcr_actions_summerwind_dev)
- [addons.cluster.x-k8s.io](https://crates.io/crates/kcr_addons_cluster_x_k8s_io)
- [agent.k8s.elastic.co](https://crates.io/crates/kcr_agent_k8s_elastic_co)
- [akri.sh](https://crates.io/crates/kcr_akri_sh)
- [amd.com](https://crates.io/crates/kcr_amd_com)
- [anywhere.eks.amazonaws.com](https://crates.io/crates/kcr_anywhere_eks_amazonaws_com)
- [apacheweb.arsenal.dev](https://crates.io/crates/kcr_apacheweb_arsenal_dev)
- [api.clever-cloud.com](https://crates.io/crates/kcr_api_clever_cloud_com)
- [api.kubemod.io](https://crates.io/crates/kcr_api_kubemod_io)
- [apicodegen.apimatic.io](https://crates.io/crates/kcr_apicodegen_apimatic_io)
- [apiextensions.crossplane.io](https://crates.io/crates/kcr_apiextensions_crossplane_io)
- [apigatewayv2.services.k8s.aws](https://crates.io/crates/kcr_apigatewayv2_services_k8s_aws)
- [apisix.apache.org](https://crates.io/crates/kcr_apisix_apache_org)
- [apm.k8s.elastic.co](https://crates.io/crates/kcr_apm_k8s_elastic_co)
- [app.k8s.io](https://crates.io/crates/kcr_app_k8s_io)
- [app.kiegroup.org](https://crates.io/crates/kcr_app_kiegroup_org)
- [app.lightbend.com](https://crates.io/crates/kcr_app_lightbend_com)
- [app.redislabs.com](https://crates.io/crates/kcr_app_redislabs_com)
- [app.terraform.io](https://crates.io/crates/kcr_app_terraform_io)
- [application-networking.k8s.aws](https://crates.io/crates/kcr_application_networking_k8s_aws)
- [applicationautoscaling.services.k8s.aws](https://crates.io/crates/kcr_applicationautoscaling_services_k8s_aws)
- [appmesh.k8s.aws](https://crates.io/crates/kcr_appmesh_k8s_aws)
- [appprotect.f5.com](https://crates.io/crates/kcr_appprotect_f5_com)
- [appprotectdos.f5.com](https://crates.io/crates/kcr_appprotectdos_f5_com)
- [apps.3scale.net](https://crates.io/crates/kcr_apps_3scale_net)
- [apps.clusternet.io](https://crates.io/crates/kcr_apps_clusternet_io)
- [apps.emqx.io](https://crates.io/crates/kcr_apps_emqx_io)
- [apps.gitlab.com](https://crates.io/crates/kcr_apps_gitlab_com)
- [apps.kubeblocks.io](https://crates.io/crates/kcr_apps_kubeblocks_io)
- [apps.kubedl.io](https://crates.io/crates/kcr_apps_kubedl_io)
- [apps.kubeedge.io](https://crates.io/crates/kcr_apps_kubeedge_io)
- [apps.m88i.io](https://crates.io/crates/kcr_apps_m88i_io)
- [apps.redhat.com](https://crates.io/crates/kcr_apps_redhat_com)
- [aquasecurity.github.io](https://crates.io/crates/kcr_aquasecurity_github_io)
- [argoproj.io](https://crates.io/crates/kcr_argoproj_io)
- [asdb.aerospike.com](https://crates.io/crates/kcr_asdb_aerospike_com)
- [atlasmap.io](https://crates.io/crates/kcr_atlasmap_io)
- [auth.ops42.org](https://crates.io/crates/kcr_auth_ops42_org)
- [authentication.stackable.tech](https://crates.io/crates/kcr_authentication_stackable_tech)
- [authzed.com](https://crates.io/crates/kcr_authzed_com)
- [automation.kubensync.com](https://crates.io/crates/kcr_automation_kubensync_com)
- [autoscaling.k8s.io](https://crates.io/crates/kcr_autoscaling_k8s_io)
- [autoscaling.karmada.io](https://crates.io/crates/kcr_autoscaling_karmada_io)
- [awx.ansible.com](https://crates.io/crates/kcr_awx_ansible_com)
- [azure.microsoft.com](https://crates.io/crates/kcr_azure_microsoft_com)
- [b3scale.infra.run](https://crates.io/crates/kcr_b3scale_infra_run)
- [b3scale.io](https://crates.io/crates/kcr_b3scale_io)
- [batch.volcano.sh](https://crates.io/crates/kcr_batch_volcano_sh)
- [beat.k8s.elastic.co](https://crates.io/crates/kcr_beat_k8s_elastic_co)
- [beegfs.csi.netapp.com](https://crates.io/crates/kcr_beegfs_csi_netapp_com)
- [binding.operators.coreos.com](https://crates.io/crates/kcr_binding_operators_coreos_com)
- [bitnami.com](https://crates.io/crates/kcr_bitnami_com)
- [bmc.tinkerbell.org](https://crates.io/crates/kcr_bmc_tinkerbell_org)
- [boskos.k8s.io](https://crates.io/crates/kcr_boskos_k8s_io)
- [bpfman.io](https://crates.io/crates/kcr_bpfman_io)
- [bus.volcano.sh](https://crates.io/crates/kcr_bus_volcano_sh)
- [cache.kubedl.io](https://crates.io/crates/kcr_cache_kubedl_io)
- [caching.ibm.com](https://crates.io/crates/kcr_caching_ibm_com)
- [camel.apache.org](https://crates.io/crates/kcr_camel_apache_org)
- [capabilities.3scale.net](https://crates.io/crates/kcr_capabilities_3scale_net)
- [capsule.clastix.io](https://crates.io/crates/kcr_capsule_clastix_io)
- [cassandra.datastax.com](https://crates.io/crates/kcr_cassandra_datastax_com)
- [ceph.rook.io](https://crates.io/crates/kcr_ceph_rook_io)
- [cert-manager.io](https://crates.io/crates/kcr_cert_manager_io)
- [certman.managed.openshift.io](https://crates.io/crates/kcr_certman_managed_openshift_io)
- [chainsaw.kyverno.io](https://crates.io/crates/kcr_chainsaw_kyverno_io)
- [chaos-mesh.org](https://crates.io/crates/kcr_chaos_mesh_org)
- [chaosblade.io](https://crates.io/crates/kcr_chaosblade_io)
- [charts.amd.com](https://crates.io/crates/kcr_charts_amd_com)
- [charts.flagsmith.com](https://crates.io/crates/kcr_charts_flagsmith_com)
- [charts.helm.k8s.io](https://crates.io/crates/kcr_charts_helm_k8s_io)
- [charts.opdev.io](https://crates.io/crates/kcr_charts_opdev_io)
- [charts.operatorhub.io](https://crates.io/crates/kcr_charts_operatorhub_io)
- [che.eclipse.org](https://crates.io/crates/kcr_che_eclipse_org)
- [chisel-operator.io](https://crates.io/crates/kcr_chisel_operator_io)
- [cilium.io](https://crates.io/crates/kcr_cilium_io)
- [claudie.io](https://crates.io/crates/kcr_claudie_io)
- [cloudformation.linki.space](https://crates.io/crates/kcr_cloudformation_linki_space)
- [cloudfront.services.k8s.aws](https://crates.io/crates/kcr_cloudfront_services_k8s_aws)
- [cloudtrail.services.k8s.aws](https://crates.io/crates/kcr_cloudtrail_services_k8s_aws)
- [cloudwatch.aws.amazon.com](https://crates.io/crates/kcr_cloudwatch_aws_amazon_com)
- [cloudwatch.services.k8s.aws](https://crates.io/crates/kcr_cloudwatch_services_k8s_aws)
- [cloudwatchlogs.services.k8s.aws](https://crates.io/crates/kcr_cloudwatchlogs_services_k8s_aws)
- [cluster.clusterpedia.io](https://crates.io/crates/kcr_cluster_clusterpedia_io)
- [cluster.ipfs.io](https://crates.io/crates/kcr_cluster_ipfs_io)
- [cluster.x-k8s.io](https://crates.io/crates/kcr_cluster_x_k8s_io)
- [clusters.clusternet.io](https://crates.io/crates/kcr_clusters_clusternet_io)
- [clustertemplate.openshift.io](https://crates.io/crates/kcr_clustertemplate_openshift_io)
- [confidentialcontainers.org](https://crates.io/crates/kcr_confidentialcontainers_org)
- [config.gatekeeper.sh](https://crates.io/crates/kcr_config_gatekeeper_sh)
- [config.grafana.com](https://crates.io/crates/kcr_config_grafana_com)
- [config.karmada.io](https://crates.io/crates/kcr_config_karmada_io)
- [config.koordinator.sh](https://crates.io/crates/kcr_config_koordinator_sh)
- [config.storageos.com](https://crates.io/crates/kcr_config_storageos_com)
- [control.k8ssandra.io](https://crates.io/crates/kcr_control_k8ssandra_io)
- [core.kubeadmiral.io](https://crates.io/crates/kcr_core_kubeadmiral_io)
- [core.linuxsuren.github.com](https://crates.io/crates/kcr_core_linuxsuren_github_com)
- [core.openfeature.dev](https://crates.io/crates/kcr_core_openfeature_dev)
- [couchbase.com](https://crates.io/crates/kcr_couchbase_com)
- [craftypath.github.io](https://crates.io/crates/kcr_craftypath_github_io)
- [crane.konveyor.io](https://crates.io/crates/kcr_crane_konveyor_io)
- [crd.projectcalico.org](https://crates.io/crates/kcr_crd_projectcalico_org)
- [data.fluid.io](https://crates.io/crates/kcr_data_fluid_io)
- [databases.schemahero.io](https://crates.io/crates/kcr_databases_schemahero_io)
- [databases.spotahome.com](https://crates.io/crates/kcr_databases_spotahome_com)
- [datadoghq.com](https://crates.io/crates/kcr_datadoghq_com)
- [dataprotection.kubeblocks.io](https://crates.io/crates/kcr_dataprotection_kubeblocks_io)
- [designer.kaoto.io](https://crates.io/crates/kcr_designer_kaoto_io)
- [devices.kubeedge.io](https://crates.io/crates/kcr_devices_kubeedge_io)
- [devops.kubesphere.io](https://crates.io/crates/kcr_devops_kubesphere_io)
- [dex.coreos.com](https://crates.io/crates/kcr_dex_coreos_com)
- [dex.gpu-ninja.com](https://crates.io/crates/kcr_dex_gpu_ninja_com)
- [digitalis.io](https://crates.io/crates/kcr_digitalis_io)
- [documentdb.services.k8s.aws](https://crates.io/crates/kcr_documentdb_services_k8s_aws)
- [druid.apache.org](https://crates.io/crates/kcr_druid_apache_org)
- [dynamodb.services.k8s.aws](https://crates.io/crates/kcr_dynamodb_services_k8s_aws)
- [ec2.services.k8s.aws](https://crates.io/crates/kcr_ec2_services_k8s_aws)
- [ecr.services.k8s.aws](https://crates.io/crates/kcr_ecr_services_k8s_aws)
- [efs.services.k8s.aws](https://crates.io/crates/kcr_efs_services_k8s_aws)
- [egressgateway.spidernet.io](https://crates.io/crates/kcr_egressgateway_spidernet_io)
- [eks.services.k8s.aws](https://crates.io/crates/kcr_eks_services_k8s_aws)
- [elasticache.services.k8s.aws](https://crates.io/crates/kcr_elasticache_services_k8s_aws)
- [elasticsearch.k8s.elastic.co](https://crates.io/crates/kcr_elasticsearch_k8s_elastic_co)
- [elbv2.k8s.aws](https://crates.io/crates/kcr_elbv2_k8s_aws)
- [emrcontainers.services.k8s.aws](https://crates.io/crates/kcr_emrcontainers_services_k8s_aws)
- [ensembleoss.io](https://crates.io/crates/kcr_ensembleoss_io)
- [enterprisesearch.k8s.elastic.co](https://crates.io/crates/kcr_enterprisesearch_k8s_elastic_co)
- [everest.percona.com](https://crates.io/crates/kcr_everest_percona_com)
- [execution.furiko.io](https://crates.io/crates/kcr_execution_furiko_io)
- [executor.testkube.io](https://crates.io/crates/kcr_executor_testkube_io)
- [expansion.gatekeeper.sh](https://crates.io/crates/kcr_expansion_gatekeeper_sh)
- [experimental.kubeblocks.io](https://crates.io/crates/kcr_experimental_kubeblocks_io)
- [extensions.istio.io](https://crates.io/crates/kcr_extensions_istio_io)
- [extensions.kubeblocks.io](https://crates.io/crates/kcr_extensions_kubeblocks_io)
- [external-secrets.io](https://crates.io/crates/kcr_external_secrets_io)
- [externaldata.gatekeeper.sh](https://crates.io/crates/kcr_externaldata_gatekeeper_sh)
- [externaldns.k8s.io](https://crates.io/crates/kcr_externaldns_k8s_io)
- [externaldns.nginx.org](https://crates.io/crates/kcr_externaldns_nginx_org)
- [flagger.app](https://crates.io/crates/kcr_flagger_app)
- [flink.apache.org](https://crates.io/crates/kcr_flink_apache_org)
- [flow.volcano.sh](https://crates.io/crates/kcr_flow_volcano_sh)
- [flows.netobserv.io](https://crates.io/crates/kcr_flows_netobserv_io)
- [fluentbit.fluent.io](https://crates.io/crates/kcr_fluentbit_fluent_io)
- [fluentd.fluent.io](https://crates.io/crates/kcr_fluentd_fluent_io)
- [flux-framework.org](https://crates.io/crates/kcr_flux_framework_org)
- [fluxcd.controlplane.io](https://crates.io/crates/kcr_fluxcd_controlplane_io)
- [forklift.konveyor.io](https://crates.io/crates/kcr_forklift_konveyor_io)
- [fossul.io](https://crates.io/crates/kcr_fossul_io)
- [frrk8s.metallb.io](https://crates.io/crates/kcr_frrk8s_metallb_io)
- [gateway.networking.k8s.io](https://crates.io/crates/kcr_gateway_networking_k8s_io)
- [gateway.networking.x-k8s.io](https://crates.io/crates/kcr_gateway_networking_x_k8s_io)
- [gateway.nginx.org](https://crates.io/crates/kcr_gateway_nginx_org)
- [getambassador.io](https://crates.io/crates/kcr_getambassador_io)
- [gitops.hybrid-cloud-patterns.io](https://crates.io/crates/kcr_gitops_hybrid_cloud_patterns_io)
- [grafana.integreatly.org](https://crates.io/crates/kcr_grafana_integreatly_org)
- [groupsnapshot.storage.k8s.io](https://crates.io/crates/kcr_groupsnapshot_storage_k8s_io)
- [hazelcast.com](https://crates.io/crates/kcr_hazelcast_com)
- [helm.sigstore.dev](https://crates.io/crates/kcr_helm_sigstore_dev)
- [helm.toolkit.fluxcd.io](https://crates.io/crates/kcr_helm_toolkit_fluxcd_io)
- [hive.openshift.io](https://crates.io/crates/kcr_hive_openshift_io)
- [hiveinternal.openshift.io](https://crates.io/crates/kcr_hiveinternal_openshift_io)
- [hnc.x-k8s.io](https://crates.io/crates/kcr_hnc_x_k8s_io)
- [hyperfoil.io](https://crates.io/crates/kcr_hyperfoil_io)
- [hyperspike.io](https://crates.io/crates/kcr_hyperspike_io)
- [iam.services.k8s.aws](https://crates.io/crates/kcr_iam_services_k8s_aws)
- [ibmcloud.ibm.com](https://crates.io/crates/kcr_ibmcloud_ibm_com)
- [image.toolkit.fluxcd.io](https://crates.io/crates/kcr_image_toolkit_fluxcd_io)
- [imaging-ingestion.alvearie.org](https://crates.io/crates/kcr_imaging_ingestion_alvearie_org)
- [inference.kubedl.io](https://crates.io/crates/kcr_inference_kubedl_io)
- [infinispan.org](https://crates.io/crates/kcr_infinispan_org)
- [infra.contrib.fluxcd.io](https://crates.io/crates/kcr_infra_contrib_fluxcd_io)
- [infrastructure.cluster.x-k8s.io](https://crates.io/crates/kcr_infrastructure_cluster_x_k8s_io)
- [install.istio.io](https://crates.io/crates/kcr_install_istio_io)
- [installation.mattermost.com](https://crates.io/crates/kcr_installation_mattermost_com)
- [instana.io](https://crates.io/crates/kcr_instana_io)
- [integration.rock8s.com](https://crates.io/crates/kcr_integration_rock8s_com)
- [iot.eclipse.org](https://crates.io/crates/kcr_iot_eclipse_org)
- [ipam.cluster.x-k8s.io](https://crates.io/crates/kcr_ipam_cluster_x_k8s_io)
- [ipam.metal3.io](https://crates.io/crates/kcr_ipam_metal3_io)
- [isindir.github.com](https://crates.io/crates/kcr_isindir_github_com)
- [jaegertracing.io](https://crates.io/crates/kcr_jaegertracing_io)
- [jenkins.io](https://crates.io/crates/kcr_jenkins_io)
- [jobset.x-k8s.io](https://crates.io/crates/kcr_jobset_x_k8s_io)
- [jobsmanager.raczylo.com](https://crates.io/crates/kcr_jobsmanager_raczylo_com)
- [k6.io](https://crates.io/crates/kcr_k6_io)
- [k8gb.absa.oss](https://crates.io/crates/kcr_k8gb_absa_oss)
- [k8s.keycloak.org](https://crates.io/crates/kcr_k8s_keycloak_org)
- [k8s.mariadb.com](https://crates.io/crates/kcr_k8s_mariadb_com)
- [k8s.nginx.org](https://crates.io/crates/kcr_k8s_nginx_org)
- [k8s.otterize.com](https://crates.io/crates/kcr_k8s_otterize_com)
- [k8up.io](https://crates.io/crates/kcr_k8up_io)
- [kafka.banzaicloud.io](https://crates.io/crates/kcr_kafka_banzaicloud_io)
- [kafka.services.k8s.aws](https://crates.io/crates/kcr_kafka_services_k8s_aws)
- [kafka.strimzi.io](https://crates.io/crates/kcr_kafka_strimzi_io)
- [kamaji.clastix.io](https://crates.io/crates/kcr_kamaji_clastix_io)
- [karpenter.k8s.aws](https://crates.io/crates/kcr_karpenter_k8s_aws)
- [karpenter.sh](https://crates.io/crates/kcr_karpenter_sh)
- [keda.sh](https://crates.io/crates/kcr_keda_sh)
- [keycloak.k8s.reddec.net](https://crates.io/crates/kcr_keycloak_k8s_reddec_net)
- [keycloak.org](https://crates.io/crates/kcr_keycloak_org)
- [keyspaces.services.k8s.aws](https://crates.io/crates/kcr_keyspaces_services_k8s_aws)
- [kibana.k8s.elastic.co](https://crates.io/crates/kcr_kibana_k8s_elastic_co)
- [kinesis.services.k8s.aws](https://crates.io/crates/kcr_kinesis_services_k8s_aws)
- [kmm.sigs.x-k8s.io](https://crates.io/crates/kcr_kmm_sigs_x_k8s_io)
- [kms.services.k8s.aws](https://crates.io/crates/kcr_kms_services_k8s_aws)
- [kom.kaiso.github.io](https://crates.io/crates/kcr_kom_kaiso_github_io)
- [kuadrant.io](https://crates.io/crates/kcr_kuadrant_io)
- [kube-green.com](https://crates.io/crates/kcr_kube_green_com)
- [kubean.io](https://crates.io/crates/kcr_kubean_io)
- [kubecost.com](https://crates.io/crates/kcr_kubecost_com)
- [kubevious.io](https://crates.io/crates/kcr_kubevious_io)
- [kueue.x-k8s.io](https://crates.io/crates/kcr_kueue_x_k8s_io)
- [kuma.io](https://crates.io/crates/kcr_kuma_io)
- [kustomize.toolkit.fluxcd.io](https://crates.io/crates/kcr_kustomize_toolkit_fluxcd_io)
- [kyverno.io](https://crates.io/crates/kcr_kyverno_io)
- [lambda.services.k8s.aws](https://crates.io/crates/kcr_lambda_services_k8s_aws)
- [leaksignal.com](https://crates.io/crates/kcr_leaksignal_com)
- [lerentis.uploadfilter24.eu](https://crates.io/crates/kcr_lerentis_uploadfilter24_eu)
- [limitador.kuadrant.io](https://crates.io/crates/kcr_limitador_kuadrant_io)
- [listeners.stackable.tech](https://crates.io/crates/kcr_listeners_stackable_tech)
- [litmuschaos.io](https://crates.io/crates/kcr_litmuschaos_io)
- [logging-extensions.banzaicloud.io](https://crates.io/crates/kcr_logging_extensions_banzaicloud_io)
- [logging.banzaicloud.io](https://crates.io/crates/kcr_logging_banzaicloud_io)
- [logstash.k8s.elastic.co](https://crates.io/crates/kcr_logstash_k8s_elastic_co)
- [loki.grafana.com](https://crates.io/crates/kcr_loki_grafana_com)
- [longhorn.io](https://crates.io/crates/kcr_longhorn_io)
- [m4e.krestomat.io](https://crates.io/crates/kcr_m4e_krestomat_io)
- [machine-deletion-remediation.medik8s.io](https://crates.io/crates/kcr_machine_deletion_remediation_medik8s_io)
- [maps.k8s.elastic.co](https://crates.io/crates/kcr_maps_k8s_elastic_co)
- [mariadb.mmontes.io](https://crates.io/crates/kcr_mariadb_mmontes_io)
- [mariadb.persistentsys](https://crates.io/crates/kcr_mariadb_persistentsys)
- [marin3r.3scale.net](https://crates.io/crates/kcr_marin3r_3scale_net)
- [mattermost.com](https://crates.io/crates/kcr_mattermost_com)
- [memorydb.services.k8s.aws](https://crates.io/crates/kcr_memorydb_services_k8s_aws)
- [metacontroller.k8s.io](https://crates.io/crates/kcr_metacontroller_k8s_io)
- [metal3.io](https://crates.io/crates/kcr_metal3_io)
- [metallb.io](https://crates.io/crates/kcr_metallb_io)
- [microcks.github.io](https://crates.io/crates/kcr_microcks_github_io)
- [minio.min.io](https://crates.io/crates/kcr_minio_min_io)
- [mirrors.kts.studio](https://crates.io/crates/kcr_mirrors_kts_studio)
- [model.kubedl.io](https://crates.io/crates/kcr_model_kubedl_io)
- [monitoring.coreos.com](https://crates.io/crates/kcr_monitoring_coreos_com)
- [monitoring.giantswarm.io](https://crates.io/crates/kcr_monitoring_giantswarm_io)
- [monocle.monocle.change-metrics.io](https://crates.io/crates/kcr_monocle_monocle_change_metrics_io)
- [mq.services.k8s.aws](https://crates.io/crates/kcr_mq_services_k8s_aws)
- [multicluster.crd.antrea.io](https://crates.io/crates/kcr_multicluster_crd_antrea_io)
- [multicluster.x-k8s.io](https://crates.io/crates/kcr_multicluster_x_k8s_io)
- [mutations.gatekeeper.sh](https://crates.io/crates/kcr_mutations_gatekeeper_sh)
- [nativestor.alauda.io](https://crates.io/crates/kcr_nativestor_alauda_io)
- [netchecks.io](https://crates.io/crates/kcr_netchecks_io)
- [networkfirewall.services.k8s.aws](https://crates.io/crates/kcr_networkfirewall_services_k8s_aws)
- [networking.gke.io](https://crates.io/crates/kcr_networking_gke_io)
- [networking.istio.io](https://crates.io/crates/kcr_networking_istio_io)
- [networking.k8s.aws](https://crates.io/crates/kcr_networking_k8s_aws)
- [networking.karmada.io](https://crates.io/crates/kcr_networking_karmada_io)
- [nfd.k8s-sigs.io](https://crates.io/crates/kcr_nfd_k8s_sigs_io)
- [nfd.kubernetes.io](https://crates.io/crates/kcr_nfd_kubernetes_io)
- [nodeinfo.volcano.sh](https://crates.io/crates/kcr_nodeinfo_volcano_sh)
- [notebook.kubedl.io](https://crates.io/crates/kcr_notebook_kubedl_io)
- [notification.toolkit.fluxcd.io](https://crates.io/crates/kcr_notification_toolkit_fluxcd_io)
- [objectbucket.io](https://crates.io/crates/kcr_objectbucket_io)
- [objectstorage.k8s.io](https://crates.io/crates/kcr_objectstorage_k8s_io)
- [ocmagent.managed.openshift.io](https://crates.io/crates/kcr_ocmagent_managed_openshift_io)
- [onepassword.com](https://crates.io/crates/kcr_onepassword_com)
- [opensearch.opster.io](https://crates.io/crates/kcr_opensearch_opster_io)
- [opensearchservice.services.k8s.aws](https://crates.io/crates/kcr_opensearchservice_services_k8s_aws)
- [opentelemetry.io](https://crates.io/crates/kcr_opentelemetry_io)
- [operations.kubeblocks.io](https://crates.io/crates/kcr_operations_kubeblocks_io)
- [operations.kubeedge.io](https://crates.io/crates/kcr_operations_kubeedge_io)
- [operator.aquasec.com](https://crates.io/crates/kcr_operator_aquasec_com)
- [operator.authorino.kuadrant.io](https://crates.io/crates/kcr_operator_authorino_kuadrant_io)
- [operator.cluster.x-k8s.io](https://crates.io/crates/kcr_operator_cluster_x_k8s_io)
- [operator.cryostat.io](https://crates.io/crates/kcr_operator_cryostat_io)
- [operator.marin3r.3scale.net](https://crates.io/crates/kcr_operator_marin3r_3scale_net)
- [operator.open-cluster-management.io](https://crates.io/crates/kcr_operator_open_cluster_management_io)
- [operator.shipwright.io](https://crates.io/crates/kcr_operator_shipwright_io)
- [operator.tekton.dev](https://crates.io/crates/kcr_operator_tekton_dev)
- [operator.tigera.io](https://crates.io/crates/kcr_operator_tigera_io)
- [operator.victoriametrics.com](https://crates.io/crates/kcr_operator_victoriametrics_com)
- [oracle.db.anthosapis.com](https://crates.io/crates/kcr_oracle_db_anthosapis_com)
- [org.eclipse.che](https://crates.io/crates/kcr_org_eclipse_che)
- [organizations.services.k8s.aws](https://crates.io/crates/kcr_organizations_services_k8s_aws)
- [parameters.kubeblocks.io](https://crates.io/crates/kcr_parameters_kubeblocks_io)
- [perses.dev](https://crates.io/crates/kcr_perses_dev)
- [pgv2.percona.com](https://crates.io/crates/kcr_pgv2_percona_com)
- [pipes.services.k8s.aws](https://crates.io/crates/kcr_pipes_services_k8s_aws)
- [pkg.crossplane.io](https://crates.io/crates/kcr_pkg_crossplane_io)
- [policies.kyverno.io](https://crates.io/crates/kcr_policies_kyverno_io)
- [policy.clusterpedia.io](https://crates.io/crates/kcr_policy_clusterpedia_io)
- [policy.karmada.io](https://crates.io/crates/kcr_policy_karmada_io)
- [policy.kubeedge.io](https://crates.io/crates/kcr_policy_kubeedge_io)
- [policy.networking.k8s.io](https://crates.io/crates/kcr_policy_networking_k8s_io)
- [postgres-operator.crunchydata.com](https://crates.io/crates/kcr_postgres_operator_crunchydata_com)
- [postgresql.cnpg.io](https://crates.io/crates/kcr_postgresql_cnpg_io)
- [projectcontour.io](https://crates.io/crates/kcr_projectcontour_io)
- [prometheusservice.services.k8s.aws](https://crates.io/crates/kcr_prometheusservice_services_k8s_aws)
- [ps.percona.com](https://crates.io/crates/kcr_ps_percona_com)
- [psmdb.percona.com](https://crates.io/crates/kcr_psmdb_percona_com)
- [ptp.openshift.io](https://crates.io/crates/kcr_ptp_openshift_io)
- [pxc.percona.com](https://crates.io/crates/kcr_pxc_percona_com)
- [quay.redhat.com](https://crates.io/crates/kcr_quay_redhat_com)
- [quota.codeflare.dev](https://crates.io/crates/kcr_quota_codeflare_dev)
- [ray.io](https://crates.io/crates/kcr_ray_io)
- [rbacmanager.reactiveops.io](https://crates.io/crates/kcr_rbacmanager_reactiveops_io)
- [rc.app.stacks](https://crates.io/crates/kcr_rc_app_stacks)
- [rds.services.k8s.aws](https://crates.io/crates/kcr_rds_services_k8s_aws)
- [redhatcop.redhat.io](https://crates.io/crates/kcr_redhatcop_redhat_io)
- [registry.apicur.io](https://crates.io/crates/kcr_registry_apicur_io)
- [registry.devfile.io](https://crates.io/crates/kcr_registry_devfile_io)
- [reliablesyncs.kubeedge.io](https://crates.io/crates/kcr_reliablesyncs_kubeedge_io)
- [remediation.medik8s.io](https://crates.io/crates/kcr_remediation_medik8s_io)
- [repo-manager.pulpproject.org](https://crates.io/crates/kcr_repo_manager_pulpproject_org)
- [reports.kyverno.io](https://crates.io/crates/kcr_reports_kyverno_io)
- [reports.x-k8s.io](https://crates.io/crates/kcr_reports_x_k8s_io)
- [resources.teleport.dev](https://crates.io/crates/kcr_resources_teleport_dev)
- [rocketmq.apache.org](https://crates.io/crates/kcr_rocketmq_apache_org)
- [route53.services.k8s.aws](https://crates.io/crates/kcr_route53_services_k8s_aws)
- [route53resolver.services.k8s.aws](https://crates.io/crates/kcr_route53resolver_services_k8s_aws)
- [rules.kubeedge.io](https://crates.io/crates/kcr_rules_kubeedge_io)
- [runtime.cluster.x-k8s.io](https://crates.io/crates/kcr_runtime_cluster_x_k8s_io)
- [s3.services.k8s.aws](https://crates.io/crates/kcr_s3_services_k8s_aws)
- [s3.snappcloud.io](https://crates.io/crates/kcr_s3_snappcloud_io)
- [sagemaker.services.k8s.aws](https://crates.io/crates/kcr_sagemaker_services_k8s_aws)
- [scheduling.koordinator.sh](https://crates.io/crates/kcr_scheduling_koordinator_sh)
- [scheduling.sigs.k8s.io](https://crates.io/crates/kcr_scheduling_sigs_k8s_io)
- [scheduling.volcano.sh](https://crates.io/crates/kcr_scheduling_volcano_sh)
- [schemas.schemahero.io](https://crates.io/crates/kcr_schemas_schemahero_io)
- [scylla.scylladb.com](https://crates.io/crates/kcr_scylla_scylladb_com)
- [secretgenerator.mittwald.de](https://crates.io/crates/kcr_secretgenerator_mittwald_de)
- [secrets-store.csi.x-k8s.io](https://crates.io/crates/kcr_secrets_store_csi_x_k8s_io)
- [secrets.crossplane.io](https://crates.io/crates/kcr_secrets_crossplane_io)
- [secrets.doppler.com](https://crates.io/crates/kcr_secrets_doppler_com)
- [secrets.hashicorp.com](https://crates.io/crates/kcr_secrets_hashicorp_com)
- [secrets.stackable.tech](https://crates.io/crates/kcr_secrets_stackable_tech)
- [secretsmanager.services.k8s.aws](https://crates.io/crates/kcr_secretsmanager_services_k8s_aws)
- [secscan.quay.redhat.com](https://crates.io/crates/kcr_secscan_quay_redhat_com)
- [security-profiles-operator.x-k8s.io](https://crates.io/crates/kcr_security_profiles_operator_x_k8s_io)
- [security.istio.io](https://crates.io/crates/kcr_security_istio_io)
- [self-node-remediation.medik8s.io](https://crates.io/crates/kcr_self_node_remediation_medik8s_io)
- [sematext.com](https://crates.io/crates/kcr_sematext_com)
- [servicebinding.io](https://crates.io/crates/kcr_servicebinding_io)
- [servicemesh.cisco.com](https://crates.io/crates/kcr_servicemesh_cisco_com)
- [services.k8s.aws](https://crates.io/crates/kcr_services_k8s_aws)
- [serving.kubedl.io](https://crates.io/crates/kcr_serving_kubedl_io)
- [sfn.services.k8s.aws](https://crates.io/crates/kcr_sfn_services_k8s_aws)
- [site.superedge.io](https://crates.io/crates/kcr_site_superedge_io)
- [slinky.slurm.net](https://crates.io/crates/kcr_slinky_slurm_net)
- [slo.koordinator.sh](https://crates.io/crates/kcr_slo_koordinator_sh)
- [sloth.slok.dev](https://crates.io/crates/kcr_sloth_slok_dev)
- [snapscheduler.backube](https://crates.io/crates/kcr_snapscheduler_backube)
- [snapshot.storage.k8s.io](https://crates.io/crates/kcr_snapshot_storage_k8s_io)
- [sns.services.k8s.aws](https://crates.io/crates/kcr_sns_services_k8s_aws)
- [sonataflow.org](https://crates.io/crates/kcr_sonataflow_org)
- [source.toolkit.fluxcd.io](https://crates.io/crates/kcr_source_toolkit_fluxcd_io)
- [sparkoperator.k8s.io](https://crates.io/crates/kcr_sparkoperator_k8s_io)
- [spv.no](https://crates.io/crates/kcr_spv_no)
- [sqs.services.k8s.aws](https://crates.io/crates/kcr_sqs_services_k8s_aws)
- [sriovnetwork.openshift.io](https://crates.io/crates/kcr_sriovnetwork_openshift_io)
- [status.gatekeeper.sh](https://crates.io/crates/kcr_status_gatekeeper_sh)
- [storage.kubeblocks.io](https://crates.io/crates/kcr_storage_kubeblocks_io)
- [storageos.com](https://crates.io/crates/kcr_storageos_com)
- [sts.min.io](https://crates.io/crates/kcr_sts_min_io)
- [stunner.l7mp.io](https://crates.io/crates/kcr_stunner_l7mp_io)
- [submariner.io](https://crates.io/crates/kcr_submariner_io)
- [superset.stackable.tech](https://crates.io/crates/kcr_superset_stackable_tech)
- [telemetry.istio.io](https://crates.io/crates/kcr_telemetry_istio_io)
- [templates.gatekeeper.sh](https://crates.io/crates/kcr_templates_gatekeeper_sh)
- [tempo.grafana.com](https://crates.io/crates/kcr_tempo_grafana_com)
- [temporal.io](https://crates.io/crates/kcr_temporal_io)
- [tests.testkube.io](https://crates.io/crates/kcr_tests_testkube_io)
- [tf.tungsten.io](https://crates.io/crates/kcr_tf_tungsten_io)
- [theketch.io](https://crates.io/crates/kcr_theketch_io)
- [tinkerbell.org](https://crates.io/crates/kcr_tinkerbell_org)
- [topology.node.k8s.io](https://crates.io/crates/kcr_topology_node_k8s_io)
- [topolvm.cybozu.com](https://crates.io/crates/kcr_topolvm_cybozu_com)
- [trace.kubeblocks.io](https://crates.io/crates/kcr_trace_kubeblocks_io)
- [traefik.io](https://crates.io/crates/kcr_traefik_io)
- [training.kubedl.io](https://crates.io/crates/kcr_training_kubedl_io)
- [trident.netapp.io](https://crates.io/crates/kcr_trident_netapp_io)
- [trino.stackable.tech](https://crates.io/crates/kcr_trino_stackable_tech)
- [trust.cert-manager.io](https://crates.io/crates/kcr_trust_cert_manager_io)
- [upgrade.cattle.io](https://crates.io/crates/kcr_upgrade_cattle_io)
- [upgrade.managed.openshift.io](https://crates.io/crates/kcr_upgrade_managed_openshift_io)
- [velero.io](https://crates.io/crates/kcr_velero_io)
- [virt.virtink.smartx.com](https://crates.io/crates/kcr_virt_virtink_smartx_com)
- [volsync.backube](https://crates.io/crates/kcr_volsync_backube)
- [vpcresources.k8s.aws](https://crates.io/crates/kcr_vpcresources_k8s_aws)
- [wgpolicyk8s.io](https://crates.io/crates/kcr_wgpolicyk8s_io)
- [wildfly.org](https://crates.io/crates/kcr_wildfly_org)
- [work.karmada.io](https://crates.io/crates/kcr_work_karmada_io)
- [workload.codeflare.dev](https://crates.io/crates/kcr_workload_codeflare_dev)
- [workloads.kubeblocks.io](https://crates.io/crates/kcr_workloads_kubeblocks_io)
- [workspace.maistra.io](https://crates.io/crates/kcr_workspace_maistra_io)
- [zonecontrol.k8s.aws](https://crates.io/crates/kcr_zonecontrol_k8s_aws)
- [zookeeper.pravega.io](https://crates.io/crates/kcr_zookeeper_pravega_io)
- [zookeeper.stackable.tech](https://crates.io/crates/kcr_zookeeper_stackable_tech)
