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
pub struct Waypoint {
  /// The location along the route that the waypoint is closest to
  #[serde(rename = "latlng")]
  latlng: Option<::models::LatLng>,
  /// A location off of the route that the waypoint is (optional)
  #[serde(rename = "target_latlng")]
  target_latlng: Option<::models::LatLng>,
  /// Categories that the waypoint belongs to
  #[serde(rename = "categories")]
  categories: Option<Vec<String>>,
  /// A title for the waypoint
  #[serde(rename = "title")]
  title: Option<String>,
  /// A description of the waypoint (optional)
  #[serde(rename = "description")]
  description: Option<String>,
  /// The number meters along the route that the waypoint is located
  #[serde(rename = "distance_into_route")]
  distance_into_route: Option<i32>
}

impl Waypoint {
  pub fn new() -> Waypoint {
    Waypoint {
      latlng: None,
      target_latlng: None,
      categories: None,
      title: None,
      description: None,
      distance_into_route: None
    }
  }

  pub fn set_latlng(&mut self, latlng: ::models::LatLng) {
    self.latlng = Some(latlng);
  }

  pub fn with_latlng(mut self, latlng: ::models::LatLng) -> Waypoint {
    self.latlng = Some(latlng);
    self
  }

  pub fn latlng(&self) -> Option<&::models::LatLng> {
    self.latlng.as_ref()
  }

  pub fn reset_latlng(&mut self) {
    self.latlng = None;
  }

  pub fn set_target_latlng(&mut self, target_latlng: ::models::LatLng) {
    self.target_latlng = Some(target_latlng);
  }

  pub fn with_target_latlng(mut self, target_latlng: ::models::LatLng) -> Waypoint {
    self.target_latlng = Some(target_latlng);
    self
  }

  pub fn target_latlng(&self) -> Option<&::models::LatLng> {
    self.target_latlng.as_ref()
  }

  pub fn reset_target_latlng(&mut self) {
    self.target_latlng = None;
  }

  pub fn set_categories(&mut self, categories: Vec<String>) {
    self.categories = Some(categories);
  }

  pub fn with_categories(mut self, categories: Vec<String>) -> Waypoint {
    self.categories = Some(categories);
    self
  }

  pub fn categories(&self) -> Option<&Vec<String>> {
    self.categories.as_ref()
  }

  pub fn reset_categories(&mut self) {
    self.categories = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> Waypoint {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> Waypoint {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_distance_into_route(&mut self, distance_into_route: i32) {
    self.distance_into_route = Some(distance_into_route);
  }

  pub fn with_distance_into_route(mut self, distance_into_route: i32) -> Waypoint {
    self.distance_into_route = Some(distance_into_route);
    self
  }

  pub fn distance_into_route(&self) -> Option<&i32> {
    self.distance_into_route.as_ref()
  }

  pub fn reset_distance_into_route(&mut self) {
    self.distance_into_route = None;
  }

}



