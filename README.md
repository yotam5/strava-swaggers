# Rust API client for swagger

The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 3.0.0
- Package version: 1.0.0
- Build package: io.swagger.codegen.languages.RustClientCodegen

## Installation
Put the package under your project folder and add the following in import:
```
    "./swagger"
```

## Documentation for API Endpoints

All URIs are relative to *https://www.strava.com/api/v3*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ActivitiesApi* | [**create_activity**](docs/ActivitiesApi.md#create_activity) | **Post** /activities | Create an Activity
*ActivitiesApi* | [**get_activity_by_id**](docs/ActivitiesApi.md#get_activity_by_id) | **Get** /activities/{id} | Get Activity
*ActivitiesApi* | [**get_comments_by_activity_id**](docs/ActivitiesApi.md#get_comments_by_activity_id) | **Get** /activities/{id}/comments | List Activity Comments
*ActivitiesApi* | [**get_kudoers_by_activity_id**](docs/ActivitiesApi.md#get_kudoers_by_activity_id) | **Get** /activities/{id}/kudos | List Activity Kudoers
*ActivitiesApi* | [**get_laps_by_activity_id**](docs/ActivitiesApi.md#get_laps_by_activity_id) | **Get** /activities/{id}/laps | List Activity Laps
*ActivitiesApi* | [**get_logged_in_athlete_activities**](docs/ActivitiesApi.md#get_logged_in_athlete_activities) | **Get** /athlete/activities | List Athlete Activities
*ActivitiesApi* | [**get_zones_by_activity_id**](docs/ActivitiesApi.md#get_zones_by_activity_id) | **Get** /activities/{id}/zones | Get Activity Zones
*ActivitiesApi* | [**update_activity_by_id**](docs/ActivitiesApi.md#update_activity_by_id) | **Put** /activities/{id} | Update Activity
*AthletesApi* | [**get_logged_in_athlete**](docs/AthletesApi.md#get_logged_in_athlete) | **Get** /athlete | Get Authenticated Athlete
*AthletesApi* | [**get_logged_in_athlete_zones**](docs/AthletesApi.md#get_logged_in_athlete_zones) | **Get** /athlete/zones | Get Zones
*AthletesApi* | [**get_stats**](docs/AthletesApi.md#get_stats) | **Get** /athletes/{id}/stats | Get Athlete Stats
*AthletesApi* | [**update_logged_in_athlete**](docs/AthletesApi.md#update_logged_in_athlete) | **Put** /athlete | Update Athlete
*ClubsApi* | [**get_club_activities_by_id**](docs/ClubsApi.md#get_club_activities_by_id) | **Get** /clubs/{id}/activities | List Club Activities
*ClubsApi* | [**get_club_admins_by_id**](docs/ClubsApi.md#get_club_admins_by_id) | **Get** /clubs/{id}/admins | List Club Administrators
*ClubsApi* | [**get_club_by_id**](docs/ClubsApi.md#get_club_by_id) | **Get** /clubs/{id} | Get Club
*ClubsApi* | [**get_club_members_by_id**](docs/ClubsApi.md#get_club_members_by_id) | **Get** /clubs/{id}/members | List Club Members
*ClubsApi* | [**get_logged_in_athlete_clubs**](docs/ClubsApi.md#get_logged_in_athlete_clubs) | **Get** /athlete/clubs | List Athlete Clubs
*GearsApi* | [**get_gear_by_id**](docs/GearsApi.md#get_gear_by_id) | **Get** /gear/{id} | Get Equipment
*RoutesApi* | [**get_route_as_gpx**](docs/RoutesApi.md#get_route_as_gpx) | **Get** /routes/{id}/export_gpx | Export Route GPX
*RoutesApi* | [**get_route_as_tcx**](docs/RoutesApi.md#get_route_as_tcx) | **Get** /routes/{id}/export_tcx | Export Route TCX
*RoutesApi* | [**get_route_by_id**](docs/RoutesApi.md#get_route_by_id) | **Get** /routes/{id} | Get Route
*RoutesApi* | [**get_routes_by_athlete_id**](docs/RoutesApi.md#get_routes_by_athlete_id) | **Get** /athletes/{id}/routes | List Athlete Routes
*SegmentEffortsApi* | [**get_efforts_by_segment_id**](docs/SegmentEffortsApi.md#get_efforts_by_segment_id) | **Get** /segment_efforts | List Segment Efforts
*SegmentEffortsApi* | [**get_segment_effort_by_id**](docs/SegmentEffortsApi.md#get_segment_effort_by_id) | **Get** /segment_efforts/{id} | Get Segment Effort
*SegmentsApi* | [**explore_segments**](docs/SegmentsApi.md#explore_segments) | **Get** /segments/explore | Explore segments
*SegmentsApi* | [**get_logged_in_athlete_starred_segments**](docs/SegmentsApi.md#get_logged_in_athlete_starred_segments) | **Get** /segments/starred | List Starred Segments
*SegmentsApi* | [**get_segment_by_id**](docs/SegmentsApi.md#get_segment_by_id) | **Get** /segments/{id} | Get Segment
*SegmentsApi* | [**star_segment**](docs/SegmentsApi.md#star_segment) | **Put** /segments/{id}/starred | Star Segment
*StreamsApi* | [**get_activity_streams**](docs/StreamsApi.md#get_activity_streams) | **Get** /activities/{id}/streams | Get Activity Streams
*StreamsApi* | [**get_route_streams**](docs/StreamsApi.md#get_route_streams) | **Get** /routes/{id}/streams | Get Route Streams
*StreamsApi* | [**get_segment_effort_streams**](docs/StreamsApi.md#get_segment_effort_streams) | **Get** /segment_efforts/{id}/streams | Get Segment Effort Streams
*StreamsApi* | [**get_segment_streams**](docs/StreamsApi.md#get_segment_streams) | **Get** /segments/{id}/streams | Get Segment Streams
*UploadsApi* | [**create_upload**](docs/UploadsApi.md#create_upload) | **Post** /uploads | Upload Activity
*UploadsApi* | [**get_upload_by_id**](docs/UploadsApi.md#get_upload_by_id) | **Get** /uploads/{uploadId} | Get Upload


## Documentation For Models

 - [ActivityStats](docs/ActivityStats.md)
 - [ActivityTotal](docs/ActivityTotal.md)
 - [ActivityType](docs/ActivityType.md)
 - [ActivityZone](docs/ActivityZone.md)
 - [AltitudeStream](docs/AltitudeStream.md)
 - [BaseStream](docs/BaseStream.md)
 - [CadenceStream](docs/CadenceStream.md)
 - [ClubActivity](docs/ClubActivity.md)
 - [ClubAthlete](docs/ClubAthlete.md)
 - [Comment](docs/Comment.md)
 - [DetailedActivity](docs/DetailedActivity.md)
 - [DetailedAthlete](docs/DetailedAthlete.md)
 - [DetailedClub](docs/DetailedClub.md)
 - [DetailedGear](docs/DetailedGear.md)
 - [DetailedSegment](docs/DetailedSegment.md)
 - [DetailedSegmentEffort](docs/DetailedSegmentEffort.md)
 - [DistanceStream](docs/DistanceStream.md)
 - [Error](docs/Error.md)
 - [ExplorerResponse](docs/ExplorerResponse.md)
 - [ExplorerSegment](docs/ExplorerSegment.md)
 - [Fault](docs/Fault.md)
 - [HeartRateZoneRanges](docs/HeartRateZoneRanges.md)
 - [HeartrateStream](docs/HeartrateStream.md)
 - [Lap](docs/Lap.md)
 - [LatLng](docs/LatLng.md)
 - [LatLngStream](docs/LatLngStream.md)
 - [MetaActivity](docs/MetaActivity.md)
 - [MetaAthlete](docs/MetaAthlete.md)
 - [MetaClub](docs/MetaClub.md)
 - [MovingStream](docs/MovingStream.md)
 - [PhotosSummary](docs/PhotosSummary.md)
 - [PhotosSummaryPrimary](docs/PhotosSummaryPrimary.md)
 - [PolylineMap](docs/PolylineMap.md)
 - [PowerStream](docs/PowerStream.md)
 - [PowerZoneRanges](docs/PowerZoneRanges.md)
 - [Route](docs/Route.md)
 - [SmoothGradeStream](docs/SmoothGradeStream.md)
 - [SmoothVelocityStream](docs/SmoothVelocityStream.md)
 - [Split](docs/Split.md)
 - [SportType](docs/SportType.md)
 - [StreamSet](docs/StreamSet.md)
 - [SummaryActivity](docs/SummaryActivity.md)
 - [SummaryAthlete](docs/SummaryAthlete.md)
 - [SummaryClub](docs/SummaryClub.md)
 - [SummaryGear](docs/SummaryGear.md)
 - [SummaryPrSegmentEffort](docs/SummaryPrSegmentEffort.md)
 - [SummarySegment](docs/SummarySegment.md)
 - [SummarySegmentEffort](docs/SummarySegmentEffort.md)
 - [TemperatureStream](docs/TemperatureStream.md)
 - [TimeStream](docs/TimeStream.md)
 - [TimedZoneDistribution](docs/TimedZoneDistribution.md)
 - [TimedZoneRange](docs/TimedZoneRange.md)
 - [UpdatableActivity](docs/UpdatableActivity.md)
 - [Upload](docs/Upload.md)
 - [Waypoint](docs/Waypoint.md)
 - [ZoneRange](docs/ZoneRange.md)
 - [ZoneRanges](docs/ZoneRanges.md)
 - [Zones](docs/Zones.md)


## Documentation For Authorization

## strava_oauth
- **Type**: OAuth
- **Flow**: accessCode
- **Authorization URL**: https://www.strava.com/api/v3/oauth/authorize
- **Scopes**: 
 - **read**: Read public segments, public routes, public profile data, public posts, public events, club feeds, and leaderboards
 - **read_all**: Read private routes, private segments, and private events for the user
 - **profile:read_all**: Read all profile information even if the user has set their profile visibility to Followers or Only You
 - **profile:write**: Update the user's weight and Functional Threshold Power (FTP), and access to star or unstar segments on their behalf
 - **activity:read**: Read the user's activity data for activities that are visible to Everyone and Followers, excluding privacy zone data
 - **activity:read_all**: The same access as activity:read, plus privacy zone data and access to read the user's activities with visibility set to Only You
 - **activity:write**: Access to create manual activities and uploads, and access to edit any activities that are visible to the app, based on activity read access level

Example
```
	auth := context.WithValue(context.TODO(), sw.ContextAccessToken, "ACCESSTOKENSTRING")
    r, err := client.Service.Operation(auth, args)
```

Or via OAuth2 module to automatically refresh tokens and perform user authentication.
```
	import 	"golang.org/x/oauth2"

    / .. Perform OAuth2 round trip request and obtain a token .. //

    tokenSource := oauth2cfg.TokenSource(createContext(httpClient), &token)
	auth := context.WithValue(oauth2.NoContext, sw.ContextOAuth2, tokenSource)
    r, err := client.Service.Operation(auth, args)
```

## Author



