use once_cell::sync::Lazy;
use rustler::NifResult;
use std::sync::Mutex;
use uaparser::{Parser, UserAgentParser}; // Import Parser trait explicitly

// Initialize the parser once and store it globally
static PARSER: Lazy<Mutex<UserAgentParser>> = Lazy::new(|| {
    let parser = UserAgentParser::builder()
        .build_from_yaml("regexes.yaml")
        .expect("Parser creation failed");
    Mutex::new(parser)
});

rustler::init!("Elixir.UapNif", [parse_ua]);

#[rustler::nif]
fn parse_ua(user_agent: String) -> NifResult<String> {
    let parser = PARSER.lock().unwrap(); // Lock the parser for thread safety
    let result = parser.parse(&user_agent); // Now this should work
    let family = result.user_agent.family;
    Ok(family.to_string())
}
