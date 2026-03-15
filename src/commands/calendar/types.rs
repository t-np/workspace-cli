use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<EventDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<EventDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<Attendee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<Organizer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_data: Option<ConferenceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hangout_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDateTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,      // For all-day events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>, // For timed events (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendee {
    pub email: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub is_self: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_request: Option<CreateConferenceRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_points: Option<Vec<EntryPoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_solution: Option<ConferenceSolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConferenceRequest {
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_solution_key: Option<ConferenceSolutionKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConferenceRequestStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceSolutionKey {
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceRequestStatus {
    pub status_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryPoint {
    pub entry_point_type: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceSolution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<ConferenceSolutionKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    #[serde(default)]
    pub items: Vec<Event>,
    pub next_page_token: Option<String>,
    pub next_sync_token: Option<String>,
    pub summary: Option<String>,
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarList {
    #[serde(default)]
    pub items: Vec<CalendarListEntry>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarListEntry {
    pub id: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub primary: Option<bool>,
    pub access_role: Option<String>,
}

/// Minimal event format optimized for AI agents (reduced token usage)
/// Excludes: attendees, organizer, description, location, htmlLink, created, updated, recurrence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimalEvent {
    pub id: Option<String>,
    pub summary: Option<String>,
    pub start: Option<EventDateTime>,
    pub end: Option<EventDateTime>,
    pub status: Option<String>,
}

impl MinimalEvent {
    pub fn from_event(event: &Event) -> Self {
        Self {
            id: event.id.clone(),
            summary: event.summary.clone(),
            start: event.start.clone(),
            end: event.end.clone(),
            status: event.status.clone(),
        }
    }
}

/// Minimal event list response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinimalEventList {
    #[serde(default)]
    pub items: Vec<MinimalEvent>,
    pub next_page_token: Option<String>,
    pub next_sync_token: Option<String>,
}

impl MinimalEventList {
    pub fn from_event_list(list: &EventList) -> Self {
        Self {
            items: list.items.iter().map(MinimalEvent::from_event).collect(),
            next_page_token: list.next_page_token.clone(),
            next_sync_token: list.next_sync_token.clone(),
        }
    }
}
