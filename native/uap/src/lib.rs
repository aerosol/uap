use once_cell::sync::Lazy;
use rustler::NifResult;
use std::sync::Mutex;
use uaparser::{Parser, UserAgentParser};
static PARSER: Lazy<Mutex<UserAgentParser>> = Lazy::new(|| {
    let parser = UserAgentParser::builder()
        .build_from_yaml("regexes.yaml")
        .expect("Parser creation failed");
    Mutex::new(parser)
});

rustler::init!("Elixir.UapNif", [parse_ua]);

#[rustler::nif]
fn parse_ua(
    user_agent: String,
) -> NifResult<(
    String,
    Option<String>,
    Option<String>,
    String,
    Option<String>,
    String,
    Option<String>,
)> {
    let parser = PARSER.lock().unwrap();
    let result = parser.parse(&user_agent);
    Ok((
        result.device.family.to_string(),
        result.device.brand.map(|s| s.to_string()),
        result.device.model.map(|s| s.to_string()),
        result.os.family.to_string(),
        result.os.major.map(|s| s.to_string()),
        result.user_agent.family.to_string(),
        result.user_agent.major.map(|s| s.to_string()),
    ))
}
