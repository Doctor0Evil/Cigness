use serde::{Deserialize, Serialize};

/// Cigness → Public API → UI binding profile
/// This struct graph can be loaded by Virta-Sys / VSC-ARTEMIS as a JSON
/// or TOML config and used by any web/mobile client to bind sections
/// of the Cigness public dashboard to the public API defined in
/// `api/public_dashboard_api.cigness.json`. [file:1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CignessDashboardBinding {
    pub id: String,
    pub service_base_url: String,
    pub sections: SectionsBinding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionsBinding {
    pub overview: OverviewBinding,
    pub impact_map: ImpactMapBinding,
    pub partners: PartnersBinding,
    pub collaborator_room: CollaboratorRoomBinding,
    pub youth_prevention: YouthPreventionBinding,
    pub recycling: RecyclingBinding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewBinding {
    pub endpoint: String,
    pub method: HttpMethod,
    pub query_defaults: QueryDefaults,
    pub fields: OverviewFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDefaults {
    pub city: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewFields {
    pub smoke_free_sessions: String,
    pub eco_actions_logged: String,
    pub kg_plastics_recycled: String,
    pub kg_cigarette_butts_collected: String,
    pub devices_recycled: String,
    pub az_counties_covered: String,
    pub active_tobacco_free_partners: String,
    /// Optional time-series endpoint for trend graph.
    pub trend_endpoint: Option<String>,
    pub trend_metric_param: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactMapBinding {
    pub endpoint: String,
    pub method: HttpMethod,
    pub tile_array_path: String,
    pub tile_fields: ImpactTileFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactTileFields {
    pub tile_id: String,
    pub county: String,
    pub smoke_free_sessions: String,
    pub eco_actions: String,
    pub kg_plastics_recycled: String,
    pub kg_butts_collected: String,
    pub partner_presence: String,
    pub youth_coalition_presence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnersBinding {
    pub list_endpoint: String,
    pub detail_endpoint_template: String,
    pub method: HttpMethod,
    pub filter_mapping: PartnerFilterMapping,
    pub list_fields: PartnerListFields,
    pub detail_fields: PartnerDetailFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerFilterMapping {
    /// Query param name for collaboration status.
    pub status_param: String,
    /// Query param name for county filter.
    pub county_param: String,
    /// Query param name for organization type.
    pub org_type_param: String,
    /// UI filter → field mapping for client-side filters.
    pub ui_filters: Vec<UiFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiFilter {
    pub id: String,
    pub label: String,
    /// JSONPath-like expression to evaluate on the partner object.
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerListFields {
    pub partner_id: String,
    pub name: String,
    pub organization_type: String,
    pub city: String,
    pub county: String,
    pub programs_offered: String,
    pub youth_prevention_focus: String,
    pub eco_alignment_score: String,
    pub collaboration_status: String,
    pub primary_contact_channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerDetailFields {
    pub partner_root: String,
    pub collaborator_root: String,
    pub contact_public_email: String,
    pub contact_public_url: String,
    pub public_statement: String,
    pub areas_of_contribution: String,
    pub events_hosted: String,
    pub eco_events_hosted: String,
    pub youth_engagement_events: String,
    pub county_scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorRoomBinding {
    pub source: CollaboratorSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorSource {
    /// Strategy: "list_then_detail" = /partners + /partners/{id}
    pub strategy: String,
    pub list_endpoint: String,
    pub detail_endpoint_template: String,
    pub method: HttpMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouthPreventionBinding {
    pub endpoint: String,
    pub method: HttpMethod,
    pub county_param: String,
    pub fields: YouthPreventionFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouthPreventionFields {
    pub youth_sessions: String,
    pub youth_partners_active: String,
    pub school_events: String,
    pub stand_coalitions_supported: String,
    pub yac_sessions_supported: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecyclingBinding {
    pub endpoint: String,
    pub method: HttpMethod,
    pub fields: RecyclingFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecyclingFields {
    pub kg_plastics_recycled: String,
    pub kg_cigarette_butts_collected: String,
    pub devices_recycled: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
}

/// Default binding profile for Cigness public dashboard on top of the
/// `/api/public` spec already defined for the repository. [file:1]
pub fn default_cigness_dashboard_binding() -> CignessDashboardBinding {
    CignessDashboardBinding {
        id: "CIGNESS_PUBLIC_DASHBOARD_BINDINGS".into(),
        service_base_url: "/api/public".into(),
        sections: SectionsBinding {
            overview: OverviewBinding {
                endpoint: "/metrics/overview".into(),
                method: HttpMethod::GET,
                query_defaults: QueryDefaults {
                    city: "Phoenix".into(),
                    state: "AZ".into(),
                },
                fields: OverviewFields {
                    smoke_free_sessions: "$.total_smoke_free_sessions".into(),
                    eco_actions_logged: "$.eco_actions_logged".into(),
                    kg_plastics_recycled: "$.kg_plastics_recycled".into(),
                    kg_cigarette_butts_collected: "$.kg_cigarette_butts_collected".into(),
                    devices_recycled: "$.devices_recycled".into(),
                    az_counties_covered: "$.az_counties_covered".into(),
                    active_tobacco_free_partners: "$.active_tobacco_free_partners".into(),
                    trend_endpoint: Some("/metrics/overview/timeseries".into()),
                    trend_metric_param: Some("smoke_free_sessions".into()),
                },
            },
            impact_map: ImpactMapBinding {
                endpoint: "/impact/map".into(),
                method: HttpMethod::GET,
                tile_array_path: "$.tiles".into(),
                tile_fields: ImpactTileFields {
                    tile_id: "$.tile_id".into(),
                    county: "$.county".into(),
                    smoke_free_sessions: "$.smoke_free_sessions".into(),
                    eco_actions: "$.eco_actions".into(),
                    kg_plastics_recycled: "$.kg_plastics_recycled".into(),
                    kg_butts_collected: "$.kg_butts_collected".into(),
                    partner_presence: "$.partner_presence".into(),
                    youth_coalition_presence: "$.youth_coalition_presence".into(),
                },
            },
            partners: PartnersBinding {
                list_endpoint: "/partners".into(),
                detail_endpoint_template: "/partners/{partner_id}".into(),
                method: HttpMethod::GET,
                filter_mapping: PartnerFilterMapping {
                    status_param: "status".into(),
                    county_param: "county".into(),
                    org_type_param: "organization_type".into(),
                    ui_filters: vec![
                        UiFilter {
                            id: "youth".into(),
                            label: "Youth".into(),
                            expression: "$.youth_prevention_focus == true".into(),
                        },
                        UiFilter {
                            id: "eco".into(),
                            label: "Eco".into(),
                            expression: "$.eco_alignment_score >= 0.8".into(),
                        },
                        UiFilter {
                            id: "health".into(),
                            label: "Health".into(),
                            expression: "$.organization_type in [\"state_agency\",\"county_health_department\",\"university_program\",\"behavioral_health_provider\"]".into(),
                        },
                        UiFilter {
                            id: "schools".into(),
                            label: "Schools".into(),
                            expression: "$.organization_type == \"school_district\"".into(),
                        },
                    ],
                },
                list_fields: PartnerListFields {
                    partner_id: "$.partner_id".into(),
                    name: "$.name".into(),
                    organization_type: "$.organization_type".into(),
                    city: "$.city".into(),
                    county: "$.county".into(),
                    programs_offered: "$.programs_offered".into(),
                    youth_prevention_focus: "$.youth_prevention_focus".into(),
                    eco_alignment_score: "$.eco_alignment_score".into(),
                    collaboration_status: "$.collaboration_status".into(),
                    primary_contact_channel: "$.primary_contact_channel".into(),
                },
                detail_fields: PartnerDetailFields {
                    partner_root: "$.partner".into(),
                    collaborator_root: "$.collaborator_profile".into(),
                    contact_public_email: "$.collaborator_profile.contact_public_email".into(),
                    contact_public_url: "$.collaborator_profile.contact_public_url".into(),
                    public_statement: "$.collaborator_profile.public_statement".into(),
                    areas_of_contribution: "$.collaborator_profile.areas_of_contribution".into(),
                    events_hosted: "$.collaborator_profile.events_hosted".into(),
                    eco_events_hosted: "$.collaborator_profile.eco_events_hosted".into(),
                    youth_engagement_events: "$.collaborator_profile.youth_engagement_events".into(),
                    county_scope: "$.collaborator_profile.county_scope".into(),
                },
            },
            collaborator_room: CollaboratorRoomBinding {
                source: CollaboratorSource {
                    strategy: "list_then_detail".into(),
                    list_endpoint: "/partners".into(),
                    detail_endpoint_template: "/partners/{partner_id}".into(),
                    method: HttpMethod::GET,
                },
            },
            youth_prevention: YouthPreventionBinding {
                endpoint: "/metrics/youth_prevention".into(),
                method: HttpMethod::GET,
                county_param: "county".into(),
                fields: YouthPreventionFields {
                    youth_sessions: "$.youth_sessions".into(),
                    youth_partners_active: "$.youth_partners_active".into(),
                    school_events: "$.school_events".into(),
                    stand_coalitions_supported: "$.stand_coalitions_supported".into(),
                    yac_sessions_supported: "$.yac_sessions_supported".into(),
                },
            },
            recycling: RecyclingBinding {
                endpoint: "/metrics/overview".into(),
                method: HttpMethod::GET,
                fields: RecyclingFields {
                    kg_plastics_recycled: "$.kg_plastics_recycled".into(),
                    kg_cigarette_butts_collected: "$.kg_cigarette_butts_collected".into(),
                    devices_recycled: "$.devices_recycled".into(),
                },
            },
        },
    }
}
