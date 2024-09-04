/* 
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * OpenAPI spec version: 3.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lap {
  /// The unique identifier of this lap
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "activity")]
  activity: Option<::models::MetaActivity>,
  #[serde(rename = "athlete")]
  athlete: Option<::models::MetaAthlete>,
  /// The lap's average cadence
  #[serde(rename = "average_cadence")]
  average_cadence: Option<f32>,
  /// The lap's average speed
  #[serde(rename = "average_speed")]
  average_speed: Option<f32>,
  /// The lap's distance, in meters
  #[serde(rename = "distance")]
  distance: Option<f32>,
  /// The lap's elapsed time, in seconds
  #[serde(rename = "elapsed_time")]
  elapsed_time: Option<i32>,
  /// The start index of this effort in its activity's stream
  #[serde(rename = "start_index")]
  start_index: Option<i32>,
  /// The end index of this effort in its activity's stream
  #[serde(rename = "end_index")]
  end_index: Option<i32>,
  /// The index of this lap in the activity it belongs to
  #[serde(rename = "lap_index")]
  lap_index: Option<i32>,
  /// The maximum speed of this lat, in meters per second
  #[serde(rename = "max_speed")]
  max_speed: Option<f32>,
  /// The lap's moving time, in seconds
  #[serde(rename = "moving_time")]
  moving_time: Option<i32>,
  /// The name of the lap
  #[serde(rename = "name")]
  name: Option<String>,
  /// The athlete's pace zone during this lap
  #[serde(rename = "pace_zone")]
  pace_zone: Option<i32>,
  #[serde(rename = "split")]
  split: Option<i32>,
  /// The time at which the lap was started.
  #[serde(rename = "start_date")]
  start_date: Option<String>,
  /// The time at which the lap was started in the local timezone.
  #[serde(rename = "start_date_local")]
  start_date_local: Option<String>,
  /// The elevation gain of this lap, in meters
  #[serde(rename = "total_elevation_gain")]
  total_elevation_gain: Option<f32>
}

impl Lap {
  pub fn new() -> Lap {
    Lap {
      id: None,
      activity: None,
      athlete: None,
      average_cadence: None,
      average_speed: None,
      distance: None,
      elapsed_time: None,
      start_index: None,
      end_index: None,
      lap_index: None,
      max_speed: None,
      moving_time: None,
      name: None,
      pace_zone: None,
      split: None,
      start_date: None,
      start_date_local: None,
      total_elevation_gain: None
    }
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Lap {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_activity(&mut self, activity: ::models::MetaActivity) {
    self.activity = Some(activity);
  }

  pub fn with_activity(mut self, activity: ::models::MetaActivity) -> Lap {
    self.activity = Some(activity);
    self
  }

  pub fn activity(&self) -> Option<&::models::MetaActivity> {
    self.activity.as_ref()
  }

  pub fn reset_activity(&mut self) {
    self.activity = None;
  }

  pub fn set_athlete(&mut self, athlete: ::models::MetaAthlete) {
    self.athlete = Some(athlete);
  }

  pub fn with_athlete(mut self, athlete: ::models::MetaAthlete) -> Lap {
    self.athlete = Some(athlete);
    self
  }

  pub fn athlete(&self) -> Option<&::models::MetaAthlete> {
    self.athlete.as_ref()
  }

  pub fn reset_athlete(&mut self) {
    self.athlete = None;
  }

  pub fn set_average_cadence(&mut self, average_cadence: f32) {
    self.average_cadence = Some(average_cadence);
  }

  pub fn with_average_cadence(mut self, average_cadence: f32) -> Lap {
    self.average_cadence = Some(average_cadence);
    self
  }

  pub fn average_cadence(&self) -> Option<&f32> {
    self.average_cadence.as_ref()
  }

  pub fn reset_average_cadence(&mut self) {
    self.average_cadence = None;
  }

  pub fn set_average_speed(&mut self, average_speed: f32) {
    self.average_speed = Some(average_speed);
  }

  pub fn with_average_speed(mut self, average_speed: f32) -> Lap {
    self.average_speed = Some(average_speed);
    self
  }

  pub fn average_speed(&self) -> Option<&f32> {
    self.average_speed.as_ref()
  }

  pub fn reset_average_speed(&mut self) {
    self.average_speed = None;
  }

  pub fn set_distance(&mut self, distance: f32) {
    self.distance = Some(distance);
  }

  pub fn with_distance(mut self, distance: f32) -> Lap {
    self.distance = Some(distance);
    self
  }

  pub fn distance(&self) -> Option<&f32> {
    self.distance.as_ref()
  }

  pub fn reset_distance(&mut self) {
    self.distance = None;
  }

  pub fn set_elapsed_time(&mut self, elapsed_time: i32) {
    self.elapsed_time = Some(elapsed_time);
  }

  pub fn with_elapsed_time(mut self, elapsed_time: i32) -> Lap {
    self.elapsed_time = Some(elapsed_time);
    self
  }

  pub fn elapsed_time(&self) -> Option<&i32> {
    self.elapsed_time.as_ref()
  }

  pub fn reset_elapsed_time(&mut self) {
    self.elapsed_time = None;
  }

  pub fn set_start_index(&mut self, start_index: i32) {
    self.start_index = Some(start_index);
  }

  pub fn with_start_index(mut self, start_index: i32) -> Lap {
    self.start_index = Some(start_index);
    self
  }

  pub fn start_index(&self) -> Option<&i32> {
    self.start_index.as_ref()
  }

  pub fn reset_start_index(&mut self) {
    self.start_index = None;
  }

  pub fn set_end_index(&mut self, end_index: i32) {
    self.end_index = Some(end_index);
  }

  pub fn with_end_index(mut self, end_index: i32) -> Lap {
    self.end_index = Some(end_index);
    self
  }

  pub fn end_index(&self) -> Option<&i32> {
    self.end_index.as_ref()
  }

  pub fn reset_end_index(&mut self) {
    self.end_index = None;
  }

  pub fn set_lap_index(&mut self, lap_index: i32) {
    self.lap_index = Some(lap_index);
  }

  pub fn with_lap_index(mut self, lap_index: i32) -> Lap {
    self.lap_index = Some(lap_index);
    self
  }

  pub fn lap_index(&self) -> Option<&i32> {
    self.lap_index.as_ref()
  }

  pub fn reset_lap_index(&mut self) {
    self.lap_index = None;
  }

  pub fn set_max_speed(&mut self, max_speed: f32) {
    self.max_speed = Some(max_speed);
  }

  pub fn with_max_speed(mut self, max_speed: f32) -> Lap {
    self.max_speed = Some(max_speed);
    self
  }

  pub fn max_speed(&self) -> Option<&f32> {
    self.max_speed.as_ref()
  }

  pub fn reset_max_speed(&mut self) {
    self.max_speed = None;
  }

  pub fn set_moving_time(&mut self, moving_time: i32) {
    self.moving_time = Some(moving_time);
  }

  pub fn with_moving_time(mut self, moving_time: i32) -> Lap {
    self.moving_time = Some(moving_time);
    self
  }

  pub fn moving_time(&self) -> Option<&i32> {
    self.moving_time.as_ref()
  }

  pub fn reset_moving_time(&mut self) {
    self.moving_time = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Lap {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_pace_zone(&mut self, pace_zone: i32) {
    self.pace_zone = Some(pace_zone);
  }

  pub fn with_pace_zone(mut self, pace_zone: i32) -> Lap {
    self.pace_zone = Some(pace_zone);
    self
  }

  pub fn pace_zone(&self) -> Option<&i32> {
    self.pace_zone.as_ref()
  }

  pub fn reset_pace_zone(&mut self) {
    self.pace_zone = None;
  }

  pub fn set_split(&mut self, split: i32) {
    self.split = Some(split);
  }

  pub fn with_split(mut self, split: i32) -> Lap {
    self.split = Some(split);
    self
  }

  pub fn split(&self) -> Option<&i32> {
    self.split.as_ref()
  }

  pub fn reset_split(&mut self) {
    self.split = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> Lap {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_start_date_local(&mut self, start_date_local: String) {
    self.start_date_local = Some(start_date_local);
  }

  pub fn with_start_date_local(mut self, start_date_local: String) -> Lap {
    self.start_date_local = Some(start_date_local);
    self
  }

  pub fn start_date_local(&self) -> Option<&String> {
    self.start_date_local.as_ref()
  }

  pub fn reset_start_date_local(&mut self) {
    self.start_date_local = None;
  }

  pub fn set_total_elevation_gain(&mut self, total_elevation_gain: f32) {
    self.total_elevation_gain = Some(total_elevation_gain);
  }

  pub fn with_total_elevation_gain(mut self, total_elevation_gain: f32) -> Lap {
    self.total_elevation_gain = Some(total_elevation_gain);
    self
  }

  pub fn total_elevation_gain(&self) -> Option<&f32> {
    self.total_elevation_gain.as_ref()
  }

  pub fn reset_total_elevation_gain(&mut self) {
    self.total_elevation_gain = None;
  }

}



