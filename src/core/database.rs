use std::path::Path;

use crate::model::{Activity, Steps, Weight};
use chrono::{DateTime, FixedOffset};

static QUERY_DATE_HAS_STEPS: &'static str = "ASK { ?activity a health:Activity ; health:activity_date '%s'; health:steps ?steps . }";
static QUERY_DATE_HAS_WEIGHT: &'static str = "ASK { ?datapoint a health:WeightMeasurement ; health:weight_date '%s'; health:weight ?weight . }";
static QUERY_ACTIVITIES_AFTER: &'static str = "SELECT ?date ?id ?calories_burned ?distance ?heart_rate_avg ?heart_rate_max ?heart_rate_min ?minutes ?steps WHERE { ?datapoint a health:Activity ; health:activity_date ?date ; health:activity_id ?id . OPTIONAL { ?datapoint health:calories_burned ?calories_burned . } OPTIONAL { ?datapoint health:distance ?distance . } OPTIONAL { ?datapoint health:hearth_rate_avg ?heart_rate_avg . } OPTIONAL { ?datapoint health:hearth_rate_min ?heart_rate_min . } OPTIONAL { ?datapoint health:hearth_rate_max ?heart_rate_max . } OPTIONAL { ?datapoint health:steps ?steps . } OPTIONAL { ?datapoint health:minutes ?minutes }  FILTER  (?date >= '%s'^^xsd:date)} ORDER BY DESC(?date)";
static QUERY_STEPS_AFTER: &'static str = "SELECT ?date ?steps WHERE { ?datapoint a health:Activity ; health:activity_date ?date ; health:steps ?steps . FILTER  (?date >= '%s'^^xsd:date)} ORDER BY ?date";
static QUERY_STEPS_ON_DAY: &'static str = "SELECT ?steps WHERE { ?datapoint a health:Activity; health:activity_date ?date ; health:steps ?steps . FILTER(?date = '%s'^^xsd:date) }";
static QUERY_WEIGHT_AFTER: &'static str = "SELECT ?date ?weight WHERE { ?datapoint a health:WeightMeasurement ; health:weight_date ?date  ; health:weight ?weight . FILTER  (?date >= '%s'^^xsd:date)} ORDER BY ?date";
static QUERY_WEIGHT_ON_DAY: &'static str = "SELECT ?weight WHERE { ?datapoint a health:WeightMeasurement; health:weight_date ?date ; health:weight ?weight . FILTER(?date = '%s'^^xsd:date) }";

#[derive(Debug)]
pub struct HealthDatabase {
    connection: tracker::SparqlConnection,
    manager: tracker::NamespaceManager,
}

impl HealthDatabase {
    pub fn new() -> Result<Self, glib::Error> {
        let store_path = glib::get_user_data_dir();
        store_path.push("health");

        let ontology_path = Path::new(crate::config::PKGDATADIR).to_path_buf();
        ontology_path.push("ontology");

        let manager = tracker::NamespaceManager::new();
        manager.add_prefix("health", "https://gitlab.gnome.org/World/health#");
        Ok(Self {
            connection: tracker::SparqlConnection::new (tracker::SparqlConnectionFlags::NONE, Some(&gio::File::new_for_path(store_path)), Some(&gio::File::new_for_path(ontology_path)), None::<&gio::Cancellable>)?,
            manager,
        })
    }

    async fn get_activities(date: Option<DateTime<FixedOffset>>) -> Result<Vec<Activity>, ()> {
        todo!();
    }

    async fn get_steps(date: Option<DateTime<FixedOffset>>) -> Result<Vec<Steps>, ()> {
        todo!();
    }

    async fn get_weights(date: Option<DateTime<FixedOffset>>) -> Result<Vec<Weight>, ()> {
        todo!();
    }

    async fn reset() -> Result<(), ()> {
        todo!();
    }

    async fn save_activity(activity: Activity) -> Result<(), ()> {
        todo!();
    }

    async fn save_weight(weight: Weight) -> Result<(), ()> {
        todo!();
    }
}
