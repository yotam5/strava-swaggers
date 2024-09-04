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
pub struct HeartRateZoneRanges {
  /// Whether the athlete has set their own custom heart rate zones
  #[serde(rename = "custom_zones")]
  custom_zones: Option<bool>,
  #[serde(rename = "zones")]
  zones: Option<::models::ZoneRanges>
}

impl HeartRateZoneRanges {
  pub fn new() -> HeartRateZoneRanges {
    HeartRateZoneRanges {
      custom_zones: None,
      zones: None
    }
  }

  pub fn set_custom_zones(&mut self, custom_zones: bool) {
    self.custom_zones = Some(custom_zones);
  }

  pub fn with_custom_zones(mut self, custom_zones: bool) -> HeartRateZoneRanges {
    self.custom_zones = Some(custom_zones);
    self
  }

  pub fn custom_zones(&self) -> Option<&bool> {
    self.custom_zones.as_ref()
  }

  pub fn reset_custom_zones(&mut self) {
    self.custom_zones = None;
  }

  pub fn set_zones(&mut self, zones: ::models::ZoneRanges) {
    self.zones = Some(zones);
  }

  pub fn with_zones(mut self, zones: ::models::ZoneRanges) -> HeartRateZoneRanges {
    self.zones = Some(zones);
    self
  }

  pub fn zones(&self) -> Option<&::models::ZoneRanges> {
    self.zones.as_ref()
  }

  pub fn reset_zones(&mut self) {
    self.zones = None;
  }

}



