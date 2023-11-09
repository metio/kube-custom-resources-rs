#!/usr/bin/env bash

# SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
# SPDX-License-Identifier: 0BSD

FILTER="${1:-}"

### Generate code with kopium
NO_DEFAULTS=(
  acme_cert_manager_io_v1/challenges
  addons_cluster_x_k8s_io_v1alpha4/clusterresourcesets
  addons_cluster_x_k8s_io_v1beta1/clusterresourcesets
  apicodegen_apimatic_io_v1beta1/apimatics
  app_kiegroup_org_v1beta1/kogitobuilds
  app_kiegroup_org_v1beta1/kogitoinfras
  app_kiegroup_org_v1beta1/kogitoruntimes
  app_kiegroup_org_v1beta1/kogitosupportingservices
  app_redislabs_com_v1/redisenterpriseclusters
  app_redislabs_com_v1alpha1/redisenterpriseclusters
  app_terraform_io_v1alpha2/workspaces
  apps_3scale_net_v1alpha1/apicasts
  apps_clusternet_io_v1alpha1/descriptions
  apps_clusternet_io_v1alpha1/globalizations
  apps_clusternet_io_v1alpha1/localizations
  apps_clusternet_io_v1alpha1/subscriptions
  apps_gitlab_com_v1beta1/gitlabs
  autoscaling_k8s_io_v1/verticalpodautoscalers
  azure_microsoft_com_v1alpha1/mysqlserveradministrators
  azure_microsoft_com_v1alpha1/rediscacheactions
  azure_microsoft_com_v1beta1/azuresqlfailovergroups
  b3scale_infra_run_v1/bbbfrontends
  binding_operators_coreos_com_v1alpha1/servicebindings
  bpfd_dev_v1alpha1/bpfprograms
  bpfd_dev_v1alpha1/kprobeprograms
  bpfd_dev_v1alpha1/tcprograms
  bpfd_dev_v1alpha1/tracepointprograms
  bpfd_dev_v1alpha1/uprobeprograms
  bpfd_dev_v1alpha1/xdpprograms
  capsule_clastix_io_v1alpha1/tenants
  capsule_clastix_io_v1beta1/tenants
  capsule_clastix_io_v1beta2/tenants
  ceph_rook_io_v1/cephbucketnotifications
  cert_manager_io_v1/certificaterequests
  cert_manager_io_v1/certificates
  cert_manager_io_v1/clusterissuers
  cert_manager_io_v1/issuers
  chaos_mesh_org_v1alpha1/awschaos
  chaos_mesh_org_v1alpha1/azurechaos
  chaos_mesh_org_v1alpha1/blockchaos
  chaos_mesh_org_v1alpha1/dnschaos
  chaos_mesh_org_v1alpha1/gcpchaos
  chaos_mesh_org_v1alpha1/httpchaos
  chaos_mesh_org_v1alpha1/iochaos
  chaos_mesh_org_v1alpha1/jvmchaos
  chaos_mesh_org_v1alpha1/kernelchaos
  chaos_mesh_org_v1alpha1/networkchaos
  chaos_mesh_org_v1alpha1/physicalmachinechaos
  chaos_mesh_org_v1alpha1/podchaos
  chaos_mesh_org_v1alpha1/schedules
  chaos_mesh_org_v1alpha1/statuschecks
  chaos_mesh_org_v1alpha1/stresschaos
  chaos_mesh_org_v1alpha1/timechaos
  chaos_mesh_org_v1alpha1/workflownodes
  chaos_mesh_org_v1alpha1/workflows
  cilium_io_v2/ciliumclusterwidenetworkpolicies
  cilium_io_v2/ciliumegressgatewaypolicies
  cilium_io_v2/ciliumnetworkpolicies
  cilium_io_v2alpha1/ciliumbgppeeringpolicies
  cilium_io_v2alpha1/ciliuml2announcementpolicies
  cilium_io_v2alpha1/ciliumloadbalancerippools
  cluster_clusterpedia_io_v1alpha2/pediaclusters
  cluster_ipfs_io_v1alpha1/ipfsclusters
  clusters_clusternet_io_v1beta1/managedclusters
  core_openfeature_dev_v1alpha1/featureflagconfigurations
  core_openfeature_dev_v1alpha2/featureflagconfigurations
  couchbase_com_v2/couchbaseclusters
  couchbase_com_v2/couchbasegroups
  couchbase_com_v2/couchbaserolebindings
  couchbase_com_v2/couchbaseusers
  data_fluid_io_v1alpha1/alluxioruntimes
  data_fluid_io_v1alpha1/databackups
  data_fluid_io_v1alpha1/dataloads
  data_fluid_io_v1alpha1/goosefsruntimes
  data_fluid_io_v1alpha1/jindoruntimes
  data_fluid_io_v1alpha1/juicefsruntimes
  data_fluid_io_v1alpha1/thinruntimes
  druid_apache_org_v1alpha1/druids
  external_secrets_io_v1alpha1/clustersecretstores
  external_secrets_io_v1alpha1/secretstores
  external_secrets_io_v1beta1/clustersecretstores
  external_secrets_io_v1beta1/secretstores
  flagger_app_v1beta1/canaries
  flows_netobserv_io_v1alpha1/flowcollectors
  flows_netobserv_io_v1beta1/flowcollectors
  flows_netobserv_io_v1beta2/flowcollectors
  flux_framework_org_v1alpha1/miniclusters
  gateway_networking_k8s_io_v1/gatewayclasses
  gateway_networking_k8s_io_v1/gateways
  gateway_networking_k8s_io_v1/httproutes
  gateway_networking_k8s_io_v1alpha2/grpcroutes
  gateway_networking_k8s_io_v1alpha2/tcproutes
  gateway_networking_k8s_io_v1alpha2/tlsroutes
  gateway_networking_k8s_io_v1alpha2/udproutes
  gateway_networking_k8s_io_v1beta1/gatewayclasses
  gateway_networking_k8s_io_v1beta1/gateways
  gateway_networking_k8s_io_v1beta1/httproutes
  gateway_nginx_org_v1alpha1/nginxgateways
  getambassador_io_v3alpha1/listeners
  getambassador_io_v3alpha1/tracingservices
  hazelcast_com_v1alpha1/maps
  helm_toolkit_fluxcd_io_v2beta1/helmreleases
  hive_openshift_io_v1/hiveconfigs
  hiveinternal_openshift_io_v1alpha1/clustersyncs
  hnc_x_k8s_io_v1alpha2/hierarchyconfigurations
  hnc_x_k8s_io_v1alpha2/hncconfigurations
  image_toolkit_fluxcd_io_v1beta1/imageupdateautomations
  image_toolkit_fluxcd_io_v1beta1/imagepolicies
  image_toolkit_fluxcd_io_v1beta1/imagerepositories
  image_toolkit_fluxcd_io_v1beta2/imagepolicies
  image_toolkit_fluxcd_io_v1beta2/imagerepositories
  infrastructure_cluster_x_k8s_io_v1beta1/vsphereclusters
  infrastructure_cluster_x_k8s_io_v1beta1/vsphereclustertemplates
  infrastructure_cluster_x_k8s_io_v1beta1/vspherefailuredomains
  jobset_x_k8s_io_v1alpha2/jobsets
  k8s_otterize_com_v1alpha2/kafkaserverconfigs
  k8s_otterize_com_v1alpha3/kafkaserverconfigs
  kafka_strimzi_io_v1alpha1/kafkausers
  kafka_strimzi_io_v1beta1/kafkausers
  kafka_strimzi_io_v1beta2/kafkabridges
  kafka_strimzi_io_v1beta2/kafkaconnects
  kafka_strimzi_io_v1beta2/kafkamirrormakers
  kafka_strimzi_io_v1beta2/kafkas
  kafka_strimzi_io_v1beta2/kafkausers
  kueue_x_k8s_io_v1beta1/admissionchecks
  kueue_x_k8s_io_v1beta1/clusterqueues
  kueue_x_k8s_io_v1beta1/localqueues
  kueue_x_k8s_io_v1beta1/workloads
  kuma_io_v1alpha1/containerpatches
  kuma_io_v1alpha1/meshaccesslogs
  kuma_io_v1alpha1/meshgatewayinstances
  kuma_io_v1alpha1/meshhttproutes
  kuma_io_v1alpha1/meshloadbalancingstrategies
  kuma_io_v1alpha1/meshproxypatches
  kuma_io_v1alpha1/meshretries
  kuma_io_v1alpha1/meshtraces
  kustomize_toolkit_fluxcd_io_v1/kustomizations
  kustomize_toolkit_fluxcd_io_v1beta1/kustomizations
  kustomize_toolkit_fluxcd_io_v1beta2/kustomizations
  kyverno_io_v1/clusterpolicies
  kyverno_io_v1/policies
  kyverno_io_v2alpha1/cleanuppolicies
  kyverno_io_v2alpha1/clustercleanuppolicies
  kyverno_io_v2beta1/cleanuppolicies
  kyverno_io_v2beta1/clustercleanuppolicies
  kyverno_io_v2beta1/clusterpolicies
  kyverno_io_v2beta1/policies
  loki_grafana_com_v1/alertingrules
  loki_grafana_com_v1/lokistacks
  loki_grafana_com_v1/recordingrules
  loki_grafana_com_v1/rulerconfigs
  loki_grafana_com_v1beta1/alertingrules
  loki_grafana_com_v1beta1/lokistacks
  loki_grafana_com_v1beta1/recordingrules
  loki_grafana_com_v1beta1/rulerconfigs
  mariadb_mmontes_io_v1alpha1/backups
  mariadb_mmontes_io_v1alpha1/connections
  mariadb_mmontes_io_v1alpha1/databases
  mariadb_mmontes_io_v1alpha1/grants
  mariadb_mmontes_io_v1alpha1/mariadbs
  mariadb_mmontes_io_v1alpha1/restores
  mariadb_mmontes_io_v1alpha1/sqljobs
  mariadb_mmontes_io_v1alpha1/users
  metal3_io_v1alpha1/hostfirmwaresettings
  metal3_io_v1alpha1/preprovisioningimages
  monitoring_coreos_com_v1/prometheuses
  monitoring_coreos_com_v1/thanosrulers
  monitoring_coreos_com_v1alpha1/alertmanagerconfigs
  monitoring_coreos_com_v1alpha1/prometheusagents
  monitoring_coreos_com_v1alpha1/scrapeconfigs
  monitoring_coreos_com_v1beta1/alertmanagerconfigs
  multicluster_x_k8s_io_v1alpha1/serviceexports
  multicluster_x_k8s_io_v1alpha1/serviceimports
  networking_karmada_io_v1alpha1/multiclusterservices
  nfd_kubernetes_io_v1/nodefeaturediscoveries
  notification_toolkit_fluxcd_io_v1/receivers
  notification_toolkit_fluxcd_io_v1beta1/alerts
  notification_toolkit_fluxcd_io_v1beta1/providers
  notification_toolkit_fluxcd_io_v1beta1/receivers
  notification_toolkit_fluxcd_io_v1beta2/alerts
  notification_toolkit_fluxcd_io_v1beta2/providers
  notification_toolkit_fluxcd_io_v1beta2/receivers
  operator_cryostat_io_v1beta1/cryostats
  operator_open_cluster_management_io_v1/clustermanagers
  operator_open_cluster_management_io_v1/klusterlets
  operator_shipwright_io_v1alpha1/shipwrightbuilds
  operator_tigera_io_v1/apiservers
  operator_tigera_io_v1/installations
  policy_clusterpedia_io_v1alpha1/clusterimportpolicies
  policy_clusterpedia_io_v1alpha1/pediaclusterlifecycles
  policy_karmada_io_v1alpha1/clusteroverridepolicies
  policy_karmada_io_v1alpha1/overridepolicies
  postgres_operator_crunchydata_com_v1beta1/pgadmins
  postgres_operator_crunchydata_com_v1beta1/pgupgrades
  postgres_operator_crunchydata_com_v1beta1/postgresclusters
  ray_io_v1/rayclusters
  ray_io_v1/rayjobs
  ray_io_v1/rayservices
  ray_io_v1alpha1/rayclusters
  ray_io_v1alpha1/rayjobs
  ray_io_v1alpha1/rayservices
  redhatcop_redhat_io_v1alpha1/groupconfigs
  redhatcop_redhat_io_v1alpha1/namespaceconfigs
  redhatcop_redhat_io_v1alpha1/userconfigs
  registry_apicur_io_v1/apicurioregistries
  registry_devfile_io_v1alpha1/clusterdevfileregistrieslists
  registry_devfile_io_v1alpha1/devfileregistries
  registry_devfile_io_v1alpha1/devfileregistrieslists
  repo_manager_pulpproject_org_v1beta2/pulpbackups
  repo_manager_pulpproject_org_v1beta2/pulprestores
  resources_teleport_dev_v1/teleportloginrules
  resources_teleport_dev_v1/teleportoktaimportrules
  resources_teleport_dev_v2/teleportsamlconnectors
  resources_teleport_dev_v2/teleportusers
  resources_teleport_dev_v3/teleportgithubconnectors
  resources_teleport_dev_v3/teleportoidcconnectors
  rules_kubeedge_io_v1/ruleendpoints
  scylla_scylladb_com_v1/scyllaclusters
  secrets_crossplane_io_v1alpha1/storeconfigs
  security_profiles_operator_x_k8s_io_v1alpha1/profilebindings
  security_profiles_operator_x_k8s_io_v1alpha1/profilerecordings
  security_profiles_operator_x_k8s_io_v1alpha1/securityprofilesoperatordaemons
  security_profiles_operator_x_k8s_io_v1alpha2/rawselinuxprofiles
  security_profiles_operator_x_k8s_io_v1beta1/seccompprofiles
  servicebinding_io_v1alpha3/servicebindings
  servicebinding_io_v1beta1/servicebindings
  services_k8s_aws_v1alpha1/fieldexports
  source_toolkit_fluxcd_io_v1beta1/buckets
  source_toolkit_fluxcd_io_v1beta1/gitrepositories
  source_toolkit_fluxcd_io_v1beta1/helmcharts
  source_toolkit_fluxcd_io_v1beta1/helmrepositories
  source_toolkit_fluxcd_io_v1beta2/buckets
  source_toolkit_fluxcd_io_v1beta2/gitrepositories
  source_toolkit_fluxcd_io_v1beta2/helmcharts
  source_toolkit_fluxcd_io_v1beta2/helmrepositories
  source_toolkit_fluxcd_io_v1beta2/ocirepositories
  sparkoperator_k8s_io_v1beta2/scheduledsparkapplications
  sparkoperator_k8s_io_v1beta2/sparkapplications
  tests_testkube_io_v1/testexecutions
  tests_testkube_io_v1/testsuiteexecutions
  tests_testkube_io_v1/testtriggers
  traefik_io_v1alpha1/ingressroutes
  virt_virtink_smartx_com_v1alpha1/virtualmachines
  wildfly_org_v1alpha1/wildflyservers
  work_karmada_io_v1alpha1/clusterresourcebindings
  work_karmada_io_v1alpha1/resourcebindings
  work_karmada_io_v1alpha2/clusterresourcebindings
  work_karmada_io_v1alpha2/resourcebindings
)
for file in $(find ./crd-catalog -name '*.yaml' -type f | LC_ALL=C sort --general-numeric-sort); do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${file}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi
  crd=$(basename "${file%.*}")
  version=$(basename "$(dirname "${file}")")
  group=$(basename "$(dirname "$(dirname "${file}")")")
  rust_crd=$(echo "${crd}" | sed -e 's/\./_/g' -e 's/-/_/g')
  rust_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
  module="${rust_group}_${version}"

  mkdir --parents "./kube-custom-resources-rs/src/${module}"

  if [[ " ${NO_DEFAULTS[*]} " =~  ${module}/${rust_crd}  ]]; then
    if ! kopium --derive PartialEq --docs --filename "${file}" >"./kube-custom-resources-rs/src/${module}/${rust_crd}.rs"; then
      echo "error in ${file}"
    fi
  else
    if ! kopium --derive Default --derive PartialEq --docs --filename "${file}" >"./kube-custom-resources-rs/src/${module}/${rust_crd}.rs"; then
      echo "error in ${file}"
    fi
  fi
done


### Remove buggy resources
BUGGY_RESOURCES=(
  acid_zalan_do_v1/operatorconfigurations # has no spec field
  acid_zalan_do_v1/postgresqls # has problems with the status field
  acid_zalan_do_v1/postgresteams # has problem with the status field
  addons_cluster_x_k8s_io_v1alpha4/clusterresourcesetbindings
  addons_cluster_x_k8s_io_v1beta1/clusterresourcesetbindings
  apiextensions_crossplane_io_v1/compositionrevisions
  apiextensions_crossplane_io_v1/compositions
  apiextensions_crossplane_io_v1beta1/compositionrevisions
  apigatewayv2_services_k8s_aws_v1alpha1/integrations
  appprotect_f5_com_v1beta1/appolicies
  apps_clusternet_io_v1alpha1/manifests
  apps_kubeedge_io_v1alpha1/edgeapplications
  apps_gitlab_com_v1beta2/runners
  apps_redhat_com_v1alpha1/clusterimpairments
  aquasecurity_github_io_v1alpha1/clusterconfigauditreports
  aquasecurity_github_io_v1alpha1/configauditreports
  argoproj_io_v1alpha1/applicationsets
  autoscaling_k8s_elastic_co_v1alpha1/elasticsearchautoscalers
  awx_ansible_com_v1beta1/awxbackups
  awx_ansible_com_v1beta1/awxrestores
  awx_ansible_com_v1beta1/awxs
  camel_apache_org_v1/integrations
  camel_apache_org_v1/integrationkits
  camel_apache_org_v1/integrationplatforms
  camel_apache_org_v1alpha1/kameletbindings
  ceph_rook_io_v1/cephclusters
  charts_flagsmith_com_v1alpha1/flagsmiths
  charts_helm_k8s_io_v1alpha1/snykmonitors
  charts_opdev_io_v1alpha1/synapses
  charts_operatorhub_io_v1alpha1/cockroachdbs
  cilium_io_v2/ciliumclusterwideenvoyconfigs
  cilium_io_v2/ciliumenvoyconfigs
  cilium_io_v2/ciliumidentities
  cilium_io_v2/ciliumlocalredirectpolicies
  core_strimzi_io_v1beta2/strimzipodsets
  crd_projectcalico_org_v1/felixconfigurations # kopium: Error: unsupported recursive array type "" for kubeNodePortRanges
  crd_projectcalico_org_v1/globalnetworkpolicies # kopium: Error: unsupported recursive array type "" for notPorts
  crd_projectcalico_org_v1/networkpolicies # kopium: Error: unsupported recursive array type "" for notPorts
  dex_coreos_com_v1/authcodes
  dex_coreos_com_v1/authrequests
  dex_coreos_com_v1/connectors
  dex_coreos_com_v1/devicerequests
  dex_coreos_com_v1/devicetokens
  dex_coreos_com_v1/oauth2clients
  dex_coreos_com_v1/offlinesessionses
  dex_coreos_com_v1/passwords
  dex_coreos_com_v1/refreshtokens
  dex_coreos_com_v1/signingkeies
  enterprise_gloo_solo_io_v1/authconfigs
  extensions_istio_io_v1alpha1/wasmplugins
  flagger_app_v1beta1/alertproviders
  flagger_app_v1beta1/metrictemplates
  fossul_io_v1/backupconfigs
  fossul_io_v1/backups
  fossul_io_v1/backupschedules
  fossul_io_v1/fossuls
  fossul_io_v1/restores
  gateway_solo_io_v1/gateways
  gateway_solo_io_v1/httpgateways
  gateway_solo_io_v1/routeoptions
  gateway_solo_io_v1/routetables
  gateway_solo_io_v1/virtualhostoptions
  gateway_solo_io_v1/virtualservices
  getambassador_io_v1/authservices
  getambassador_io_v1/consulresolvers
  getambassador_io_v1/devportals
  getambassador_io_v1/kubernetesendpointresolvers
  getambassador_io_v1/kubernetesserviceresolvers
  getambassador_io_v1/logservices
  getambassador_io_v1/mappings
  getambassador_io_v1/modules
  getambassador_io_v1/ratelimitservices
  getambassador_io_v1/tcpmappings
  getambassador_io_v1/tlscontexts
  getambassador_io_v1/tracingservices
  getambassador_io_v2/authservices
  getambassador_io_v2/consulresolvers
  getambassador_io_v2/devportals
  getambassador_io_v2/hosts
  getambassador_io_v2/kubernetesendpointresolvers
  getambassador_io_v2/kubernetesserviceresolvers
  getambassador_io_v2/logservices
  getambassador_io_v2/mappings
  getambassador_io_v2/modules
  getambassador_io_v2/ratelimitservices
  getambassador_io_v2/tcpmappings
  getambassador_io_v2/tlscontexts
  getambassador_io_v2/tracingservices
  getambassador_io_v3alpha1/mappings
  gloo_solo_io_v1/proxies
  gloo_solo_io_v1/settings
  gloo_solo_io_v1/upstreams
  gloo_solo_io_v1/upstreamgroups
  grafana_integreatly_org_v1beta1/grafanas
  graphql_gloo_solo_io_v1beta1/graphqlapis
  helm_sigstore_dev_v1alpha1/rekors
  hive_openshift_io_v1/selectorsyncsets
  hive_openshift_io_v1/syncsets
  infinispan_org_v1/infinispans
  install_istio_io_v1alpha1/istiooperators
  jobsmanager_raczylo_com_v1beta1/managedjobs
  kafka_strimzi_io_v1beta2/kafkamirrormaker2s
  kiali_io_v1alpha1/kialis
  kubean_io_v1alpha1/localartifactsets
  kuma_io_v1alpha1/circuitbreakers
  kuma_io_v1alpha1/dataplaneinsights
  kuma_io_v1alpha1/dataplanes
  kuma_io_v1alpha1/externalservices
  kuma_io_v1alpha1/faultinjections
  kuma_io_v1alpha1/healthchecks
  kuma_io_v1alpha1/meshes
  kuma_io_v1alpha1/meshgatewayroutes
  kuma_io_v1alpha1/meshgateways
  kuma_io_v1alpha1/meshinsights
  kuma_io_v1alpha1/proxytemplates
  kuma_io_v1alpha1/ratelimits
  kuma_io_v1alpha1/retries
  kuma_io_v1alpha1/serviceinsights
  kuma_io_v1alpha1/timeouts
  kuma_io_v1alpha1/trafficlogs
  kuma_io_v1alpha1/trafficpermissions
  kuma_io_v1alpha1/trafficroutes
  kuma_io_v1alpha1/traffictraces
  kuma_io_v1alpha1/virtualoutbounds
  kuma_io_v1alpha1/zoneegresses
  kuma_io_v1alpha1/zoneegressinsights
  kuma_io_v1alpha1/zoneingresses
  kuma_io_v1alpha1/zoneingressinsights
  kuma_io_v1alpha1/zoneinsights
  kuma_io_v1alpha1/zones
  lambda_services_k8s_aws_v1alpha1/aliases
  lb_lbconfig_carlosedp_com_v1/externalloadbalancers
  litmuschaos_io_v1alpha1/chaosresults
  logging_banzaicloud_io_v1beta1/fluentbitagents
  logging_banzaicloud_io_v1beta1/loggings
  logging_banzaicloud_io_v1beta1/nodeagents
  logging_extensions_banzaicloud_io_v1alpha1/eventtailers
  longhorn_io_v1beta1/backingimagedatasources
  longhorn_io_v1beta1/backingimagemanagers
  longhorn_io_v1beta1/backingimages
  longhorn_io_v1beta1/backups
  longhorn_io_v1beta1/backuptargets
  longhorn_io_v1beta1/backupvolumes
  longhorn_io_v1beta1/engineimages
  longhorn_io_v1beta1/engines
  longhorn_io_v1beta1/instancemanagers
  longhorn_io_v1beta1/nodes
  longhorn_io_v1beta1/recurringjobs
  longhorn_io_v1beta1/replicas
  longhorn_io_v1beta1/settings
  longhorn_io_v1beta1/sharemanagers
  longhorn_io_v1beta1/volumes
  longhorn_io_v1beta2/settings
  metal3_io_v1alpha1/baremetalhosts
  microcks_github_io_v1alpha1/microcksinstalls
  multicluster_x_k8s_io_v1alpha1/works
  networking_istio_io_v1alpha3/destinationrules
  networking_istio_io_v1alpha3/envoyfilters
  networking_istio_io_v1alpha3/gateways
  networking_istio_io_v1alpha3/serviceentries
  networking_istio_io_v1alpha3/sidecars
  networking_istio_io_v1alpha3/virtualservices
  networking_istio_io_v1alpha3/workloadentries
  networking_istio_io_v1alpha3/workloadgroups
  networking_istio_io_v1beta1/destinationrules
  networking_istio_io_v1beta1/gateways
  networking_istio_io_v1beta1/proxyconfigs
  networking_istio_io_v1beta1/serviceentries
  networking_istio_io_v1beta1/sidecars
  networking_istio_io_v1beta1/virtualservices
  networking_istio_io_v1beta1/workloadentries
  networking_istio_io_v1beta1/workloadgroups
  objectbucket_io_v1alpha1/objectbuckets
  objectbucket_io_v1alpha1/objectbucketclaims
  operator_knative_dev_v1beta1/knativeeventings
  operator_knative_dev_v1beta1/knativeservings
  operator_tekton_dev_v1alpha1/tektonchains
  operator_tekton_dev_v1alpha1/tektonconfigs
  operator_tekton_dev_v1alpha1/tektonhubs
  operator_tekton_dev_v1alpha1/tektoninstallersets
  operator_tekton_dev_v1alpha1/tektonpipelines
  operator_tekton_dev_v1alpha1/tektontriggers
  operator_tigera_io_v1/imagesets
  operator_victoriametrics_com_v1beta1/vmagents
  operator_victoriametrics_com_v1beta1/vmalertmanagerconfigs
  operator_victoriametrics_com_v1beta1/vmalertmanagers
  operator_victoriametrics_com_v1beta1/vmalerts
  operator_victoriametrics_com_v1beta1/vmauths
  operator_victoriametrics_com_v1beta1/vmclusters
  operator_victoriametrics_com_v1beta1/vmnodescrapes
  operator_victoriametrics_com_v1beta1/vmpodscrapes
  operator_victoriametrics_com_v1beta1/vmprobes
  operator_victoriametrics_com_v1beta1/vmservicescrapes
  operator_victoriametrics_com_v1beta1/vmsingles
  operator_victoriametrics_com_v1beta1/vmstaticscrapes
  postgresql_cnpg_io_v1/clusters
  redhatcop_redhat_io_v1alpha1/patches
  repo_manager_pulpproject_org_v1beta2/pulps
  resources_teleport_dev_v2/teleportprovisiontokens
  resources_teleport_dev_v5/teleportroles
  resources_teleport_dev_v6/teleportroles
  ripsaw_cloudbulldozer_io_v1alpha1/benchmarks
  security_istio_io_v1/authorizationpolicies
  security_istio_io_v1/requestauthentications
  security_istio_io_v1beta1/authorizationpolicies
  security_istio_io_v1beta1/peerauthentications
  security_istio_io_v1beta1/requestauthentications
  security_profiles_operator_x_k8s_io_v1alpha2/selinuxprofiles
  sematext_com_v1/sematextagents
  source_toolkit_fluxcd_io_v1/gitrepositories
  telemetry_istio_io_v1alpha1/telemetries
  traefik_io_v1alpha1/middlewares
  trident_netapp_io_v1/tridentorchestrators
  work_karmada_io_v1alpha1/works
)
for resource in "${BUGGY_RESOURCES[@]}"; do
  rm --force "./kube-custom-resources-rs/src/${resource}.rs"
done


### Generate mod.rs files
for file in $(find ./crd-catalog -name '*.yaml' -type f | LC_ALL=C sort --general-numeric-sort); do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${file}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi
  crd=$(basename "${file%.*}")
  version=$(basename "$(dirname "${file}")")
  group=$(basename "$(dirname "$(dirname "${file}")")")
  rust_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
  module="${rust_group}_${version}"

  rm --force "./kube-custom-resources-rs/src/${module}/mod.rs"
done
for file in $(find ./crd-catalog -name '*.yaml' -type f | LC_ALL=C sort --general-numeric-sort); do
  if [ -n "${FILTER}" ]; then
    if ! echo -n "${file}" | grep --quiet "${FILTER}"; then
      continue
    fi
  fi
  crd=$(basename "${file%.*}")
  version=$(basename "$(dirname "${file}")")
  group=$(basename "$(dirname "$(dirname "${file}")")")
  rust_crd=$(echo "${crd}" | sed -e 's/\./_/g' -e 's/-/_/g')
  rust_group=$(echo "${group}" | sed -e 's/\./_/g' -e 's/-/_/g')
  module="${rust_group}_${version}"

  if [ -f "./kube-custom-resources-rs/src/${module}/${rust_crd}.rs" ]; then
    echo "pub mod ${rust_crd};" >> "./kube-custom-resources-rs/src/${module}/mod.rs"
  fi
done


### Adjust Cargo.toml
sed -i '/\[features\]/,$d' ./kube-custom-resources-rs/Cargo.toml
echo '[features]' >>./kube-custom-resources-rs/Cargo.toml

for mld in $(find ./kube-custom-resources-rs/src -type d | LC_ALL=C sort --general-numeric-sort); do
  module=$(basename "${mld}")

  if [ -f "${mld}/mod.rs" ]; then
    echo "${module} = []" >>./kube-custom-resources-rs/Cargo.toml
  fi
done

