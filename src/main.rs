use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Event {
    title: String,
    #[serde(alias = "startDt")]
    start_dt: String,
    #[serde(alias = "endDt")]
    end_dt: String,
    #[serde(alias = "startY")]
    start_y: String,
    #[serde(alias = "endY")]
    end_y: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    data: Vec<Event>,
}

fn main() {
    let events = fetch_events();
    let result = compose_ical(&events);

    if !result.is_empty() {
        println!(r#"{}"#, result);
    } else {
        eprintln!("empty events");
    }
}

fn fetch_events() -> Vec<Event> {
    let base_url = "https://www.ajou.ac.kr/kr/ajou/notice-calendar.do";
    let params = "mode=calendar";
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
                r#"BEGIN:VEVENT\nSUMMARY:{}\nDTSTART:{}\nDTEND:{}\nEND:VEVENT"#,
                event.title,
                format!("{}{}", event.start_y, event.start_dt.replace("-", "")),
                format!("{}{}", event.end_y, event.end_dt.replace("-", "")),
            )
            .trim()
            .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");

    let footer = r#"END:VCALENDAR"#;

    format!(r#"{}\n{}\n{}"#, header, items, footer)
}
