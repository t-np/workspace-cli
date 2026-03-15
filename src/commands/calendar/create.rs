use crate::client::ApiClient;
use crate::error::Result;
use super::types::{
    Event, EventDateTime, Attendee, ConferenceData,
    CreateConferenceRequest, ConferenceSolutionKey, CalendarListEntry,
};

pub struct CreateEventParams {
    pub calendar_id: String,
    pub summary: String,
    pub start: String,  // RFC3339 or YYYY-MM-DD
    pub end: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub attendees: Vec<String>,
    pub time_zone: Option<String>,
}

pub async fn create_event(client: &ApiClient, params: CreateEventParams) -> Result<Event> {
    let is_all_day = !params.start.contains('T');

    let start = if is_all_day {
        EventDateTime {
            date: Some(params.start.clone()),
            date_time: None,
            time_zone: None,
        }
    } else {
        EventDateTime {
            date: None,
            date_time: Some(params.start.clone()),
            time_zone: params.time_zone.clone(),
        }
    };

    let end = if is_all_day {
        EventDateTime {
            date: Some(params.end.clone()),
            date_time: None,
            time_zone: None,
        }
    } else {
        EventDateTime {
            date: None,
            date_time: Some(params.end.clone()),
            time_zone: params.time_zone.clone(),
        }
    };

    // Get organizer email from Calendar API (primary calendar ID = user email)
    let organizer_email: Option<String> = client
        .get::<CalendarListEntry>("/users/me/calendarList/primary")
        .await
        .ok()
        .map(|cal| cal.id);

    // Build attendees — add invited guests + auto-accept organizer
    let mut attendees: Vec<Attendee> = params.attendees
        .into_iter()
        .map(|email| Attendee {
            email,
            optional: false,
            response_status: Some("needsAction".to_string()),
        })
        .collect();

    if let Some(ref email) = organizer_email {
        attendees.push(Attendee {
            email: email.clone(),
            optional: false,
            response_status: Some("accepted".to_string()),
        });
    }

    // Auto-generate Google Meet link (matches web UI default)
    let conference_data = ConferenceData {
        create_request: Some(CreateConferenceRequest {
            request_id: generate_request_id(),
            conference_solution_key: Some(ConferenceSolutionKey {
                r#type: "hangoutsMeet".to_string(),
            }),
            status: None,
        }),
        entry_points: None,
        conference_solution: None,
        conference_id: None,
    };

    let event = Event {
        id: None,
        summary: Some(params.summary),
        description: params.description,
        location: params.location,
        start: Some(start),
        end: Some(end),
        status: None,
        attendees,
        organizer: None,
        html_link: None,
        created: None,
        updated: None,
        recurrence: None,
        conference_data: Some(conference_data),
        hangout_link: None,
    };

    // conferenceDataVersion=1 enables Meet link creation
    // sendUpdates=all sends email notifications to attendees (matches web UI default)
    let path = format!(
        "/calendars/{}/events?conferenceDataVersion=1&sendUpdates=all",
        urlencoding::encode(&params.calendar_id)
    );
    let created: Event = client.post(&path, &event).await?;

    // Auto-accept organizer RSVP (API ignores responseStatus during create, needs follow-up PATCH)
    if let Some(ref event_id) = created.id {
        if let Some(ref org_email) = organizer_email {
            // Build attendees with explicit responseStatus (must always be present for PATCH)
            let patched_attendees: Vec<serde_json::Value> = created.attendees.iter().map(|a| {
                if a.email == *org_email {
                    serde_json::json!({ "email": a.email, "responseStatus": "accepted" })
                } else {
                    serde_json::json!({ "email": a.email, "responseStatus": a.response_status.as_deref().unwrap_or("needsAction") })
                }
            }).collect();

            let patch_body = serde_json::json!({ "attendees": patched_attendees });
            let patch_path = format!(
                "/calendars/{}/events/{}?sendUpdates=none",
                urlencoding::encode(&params.calendar_id),
                urlencoding::encode(event_id)
            );
            let _: serde_json::Value = client.patch(&patch_path, &patch_body).await.unwrap_or_default();
        }
    }

    Ok(created)
}

/// Generate a unique request ID for conference creation
fn generate_request_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("meet-{:016x}", nanos)
}
