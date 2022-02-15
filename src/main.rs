use chrono::{Datelike, Utc};
use itertools::Itertools;
use serde::Deserialize;
use std::{thread, time::Duration};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Event {
    #[serde(alias = "articleNo")]
    article_no: i32,
    title: String,
    start: String,
    end: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    data: Vec<Event>,
}

fn main() {
    let year = Utc::now().year();
    let events = (1..13)
        .flat_map(|month| -> Vec<Event> {
            thread::sleep(Duration::from_millis(1000));
            fetch_events(year, month)
        })
        .unique_by(|event| event.article_no)
        .collect::<Vec<Event>>();
    let result = compose_ical(&events);

    if !result.is_empty() {
        println!(r#"{}"#, result);
    } else {
        eprintln!("empty events");
    }
}

fn fetch_events(year: i32, month: i32) -> Vec<Event> {
    let base_url = "https://www.ajou.ac.kr/kr/ajou/notice-calendar.do";
    let params = format!("mode=calendar&date={}-{:02}-01", year, month);
    let url = format!("{}?{}", base_url, params);

    let response = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .unwrap();

    assert!(response.status().is_success());

    let text = response.text().unwrap();
    serde_json::from_str::<Response>(&text).unwrap().data
}

fn compose_ical(events: &[Event]) -> String {
    let header = r#"BEGIN:VCALENDAR\nVERSION:2.0\nPRODID:-//Ajou University/학사일정//KR\nCALSCALE:GREGORIAN\nMETHOD:PUBLISH"#;

    let items = events
        .iter()
        .map(|event| -> String {
            format!(
                r#"BEGIN:VEVENT\nUID:{}\nSUMMARY:{}\nDTSTART:{}\nDTEND:{}\nDTSTAMP:{}\nEND:VEVENT"#,
                Uuid::new_v4(),
                event.title,
                event.start.replace("-", ""),
                event.end.replace("-", ""),
                Utc::now().format("%Y%m%dT%H%M%SZ"),
            )
            .trim()
            .to_string()
        })
        .collect::<Vec<String>>()
        .join(r#"\n"#);

    let footer = r#"END:VCALENDAR"#;

    format!(r#"{}\n{}\n{}"#, header, items, footer)
}
