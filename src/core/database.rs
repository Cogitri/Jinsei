use std::path::Path;

use crate::model::{Activity, ActivityType, Steps, Weight};
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, Utc};
use num_traits::cast::{FromPrimitive, ToPrimitive};
use uom::si::{f32::Mass, length::meter, mass::kilogram, u32::Length};

#[derive(Debug)]
pub struct HealthDatabase {
    connection: tracker::SparqlConnection,
    manager: tracker::NamespaceManager,
}

impl HealthDatabase {
    pub fn new() -> Result<Self, glib::Error> {
        let mut store_path = glib::get_user_data_dir();
        store_path.push("health");

        let mut ontology_path = Path::new(crate::config::PKGDATADIR).to_path_buf();
        ontology_path.push("ontology");

        let manager = tracker::NamespaceManager::new();
        manager.add_prefix("health", "https://gitlab.gnome.org/World/health#");
        Ok(Self {
            connection: tracker::SparqlConnection::new(
                tracker::SparqlConnectionFlags::NONE,
                Some(&gio::File::new_for_path(store_path)),
                Some(&gio::File::new_for_path(ontology_path)),
                None::<&gio::Cancellable>,
            )?,
            manager,
        })
    }

    pub async fn get_activities(
        &self,
        date_opt: Option<DateTime<FixedOffset>>,
    ) -> Result<Vec<Activity>, glib::Error> {
        let cursor = if let Some(date) = date_opt {
            self.connection.query_async_future(&format!("SELECT ?date ?id ?calories_burned ?distance ?heart_rate_avg ?heart_rate_max ?heart_rate_min ?minutes ?steps WHERE {{ ?datapoint a health:Activity ; health:activity_date ?date ; health:activity_id ?id . OPTIONAL {{ ?datapoint health:calories_burned ?calories_burned . }} OPTIONAL {{ ?datapoint health:distance ?distance . }} OPTIONAL {{ ?datapoint health:hearth_rate_avg ?heart_rate_avg . }} OPTIONAL {{ ?datapoint health:hearth_rate_min ?heart_rate_min . }} OPTIONAL {{ ?datapoint health:hearth_rate_max ?heart_rate_max . }} OPTIONAL {{ ?datapoint health:steps ?steps . }} OPTIONAL {{ ?datapoint health:minutes ?minutes }} FILTER  (?date >= '{}'^^xsd:dateTime)}} ORDER BY DESC(?date)", date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))).await?
        } else {
            self.connection.query_async_future("SELECT ?date ?id ?calories_burned ?distance ?heart_rate_avg ?heart_rate_max ?heart_rate_min ?minutes ?steps WHERE { ?datapoint a health:Activity ; health:activity_date ?date ; health:activity_id ?id . OPTIONAL { ?datapoint health:calories_burned ?calories_burned . } OPTIONAL { ?datapoint health:distance ?distance . } OPTIONAL { ?datapoint health:hearth_rate_avg ?heart_rate_avg . } OPTIONAL { ?datapoint health:hearth_rate_min ?heart_rate_min . } OPTIONAL { ?datapoint health:hearth_rate_max ?heart_rate_max . } OPTIONAL { ?datapoint health:steps ?steps . } OPTIONAL { ?datapoint health:minutes ?minutes } } ORDER BY DESC(?date)").await?
        };

        let mut ret = Vec::new();
        while let Ok(true) = cursor.next_async_future().await {
            let activity = Activity::new();

            for i in 0..cursor.get_n_columns() {
                match cursor.get_variable_name(i).unwrap().as_str() {
                    "id" => {
                        activity.set_activity_type(
                            ActivityType::from_i64(cursor.get_integer(i)).unwrap(),
                        );
                    }
                    "date" => {
                        let datetime =
                            DateTime::parse_from_rfc3339(cursor.get_string(i).0.unwrap().as_str());
                        if datetime.is_err() {
                            // Migrate from previous date format
                            let ndt = NaiveDate::parse_from_str(
                                cursor.get_string(i).0.unwrap().as_str(),
                                "%Y-%m-%d",
                            )
                            .unwrap()
                            .and_hms(0, 0, 0);
                            activity.set_date(DateTime::<Utc>::from_utc(ndt, Utc).into());
                        } else {
                            activity.set_date(datetime.unwrap());
                        }
                    }
                    "calories_burned" => {
                        activity.set_calories_burned(Some(cursor.get_integer(i) as u32));
                    }
                    "distance" => {
                        activity
                            .set_distance(Some(Length::new::<meter>(cursor.get_integer(i) as u32)));
                    }
                    "heart_rate_avg" => {
                        activity.set_heart_rate_avg(Some(cursor.get_integer(i) as u32));
                    }
                    "heart_rate_max" => {
                        activity.set_heart_rate_max(Some(cursor.get_integer(i) as u32));
                    }
                    "heart_rate_min" => {
                        activity.set_heart_rate_min(Some(cursor.get_integer(i) as u32));
                    }
                    "minutes" => {
                        activity.set_duration(Duration::minutes(cursor.get_integer(i)));
                    }
                    "steps" => {
                        activity.set_steps(Some(cursor.get_integer(i) as u32));
                    }
                    _ => unimplemented!(),
                }
            }

            ret.push(activity);
        }

        Ok(ret)
    }

    pub async fn get_steps(&self, date: DateTime<FixedOffset>) -> Result<Vec<Steps>, glib::Error> {
        let cursor = self.connection.query_async_future(&format!("SELECT ?date ?steps WHERE {{ ?datapoint a health:Activity ; health:activity_date ?date ; health:steps ?steps . FILTER  (?date >= '{}'^^xsd:dateTime)}} ORDER BY ?date", date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))).await?;
        let mut hashmap = std::collections::HashMap::new();

        while let Ok(true) = cursor.next_async_future().await {
            let datetime = DateTime::parse_from_rfc3339(cursor.get_string(0).0.unwrap().as_str());
            let date = if datetime.is_err() {
                // Migrate from previous date format
                let ndt =
                    NaiveDate::parse_from_str(cursor.get_string(0).0.unwrap().as_str(), "%Y-%m-%d")
                        .unwrap()
                        .and_hms(0, 0, 0);
                DateTime::<Utc>::from_utc(ndt, Utc).into()
            } else {
                datetime.unwrap()
            };
            hashmap.insert(
                date,
                hashmap.get(&date).unwrap_or(&0) + cursor.get_integer(1) as u32,
            );
        }

        Ok(hashmap
            .drain()
            .map(|(date, steps)| Steps::new(date, steps))
            .collect())
    }

    pub async fn get_weights(
        &self,
        date: DateTime<FixedOffset>,
    ) -> Result<Vec<Weight>, glib::Error> {
        println!("{}", &format!("SELECT ?date ?weight WHERE {{ ?datapoint a health:WeightMeasurement ; health:weight_date ?date  ; health:weight ?weight . FILTER  (?date >= '{}'^^xsd:dateTime)}} ORDER BY ?date", date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)));
        let cursor = self.connection.query_async_future(&format!("SELECT ?date ?weight WHERE {{ ?datapoint a health:WeightMeasurement ; health:weight_date ?date  ; health:weight ?weight . FILTER  (?date >= '{}'^^xsd:dateTime)}} ORDER BY ?date", date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))).await?;
        let mut ret = Vec::new();

        while let Ok(true) = cursor.next_async_future().await {
            ret.push(Weight::new(
                DateTime::parse_from_rfc3339(cursor.get_string(0).0.unwrap().as_str()).unwrap(),
                Mass::new::<kilogram>(cursor.get_double(1) as f32),
            ));
        }

        Ok(ret)
    }

    pub async fn reset(&self) -> Result<(), glib::Error> {
        self.connection
            .update_async_future("DELETE WHERE { ?datapoint a health:WeightMeasurement }")
            .await?;
        self.connection
            .update_async_future("DELETE WHERE { ?datapoint a health:Activity }")
            .await?;

        Ok(())
    }

    pub async fn save_activity(&self, activity: Activity) -> Result<(), glib::Error> {
        let resource = tracker::Resource::new(None);
        resource.set_uri("rdf:type", "health:Activity");
        resource.set_string(
            "health:activity_date",
            &activity
                .get_date()
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        );
        resource.set_int64(
            "health:activity_id",
            activity.get_activity_type().to_u32().unwrap().into(),
        );

        if let Some(c) = activity.get_calories_burned() {
            resource.set_int64("health:calories_burned", c.into());
        }
        if let Some(d) = activity.get_distance() {
            resource.set_int64(
                "health:distance",
                d.get::<uom::si::length::kilometer>().into(),
            );
        }
        if let Some(avg) = activity.get_heart_rate_avg() {
            resource.set_int64("health:heart_rate_avg", avg.into());
        }
        if let Some(max) = activity.get_heart_rate_max() {
            resource.set_int64("health:heart_rate_max", max.into());
        }
        if let Some(min) = activity.get_heart_rate_min() {
            resource.set_int64("health:heart_rate_min", min.into());
        }
        if activity.get_duration().num_minutes() != 0 {
            resource.set_int64("health:minutes", activity.get_duration().num_minutes());
        }
        if let Some(s) = activity.get_steps() {
            resource.set_int64("health:steps", s.into());
        }

        self.connection
            .update_async_future(
                resource
                    .print_sparql_update(Some(&self.manager), None)
                    .unwrap()
                    .as_str(),
            )
            .await?;
        Ok(())
    }

    pub async fn save_weight(&self, weight: Weight) -> Result<(), glib::Error> {
        let resource = tracker::Resource::new(None);
        resource.set_uri("rdf:type", "health::WeightMeasurement");
        resource.set_string(
            "health:weight_date",
            &weight
                .date
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        );
        resource.set_double(
            "health::weight",
            weight.weight.get::<uom::si::mass::kilogram>().into(),
        );

        self.connection
            .update_async_future(&format!(
                "DELETE WHERE {{ ?u health:weight_date {} }}; {}",
                &weight
                    .date
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                resource
                    .print_sparql_update(Some(&self.manager), None)
                    .unwrap()
                    .as_str()
            ))
            .await?;
        Ok(())
    }
}
