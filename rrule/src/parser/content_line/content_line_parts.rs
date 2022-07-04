use crate::parser::{regex::get_property_name, ParseError};

use super::PropertyName;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ContentLineCaptures<'a> {
    pub property_name: PropertyName,
    pub parameters: Option<&'a str>,
    pub value: &'a str,
}

/// Get the property name, parameters and values of a content line.
pub(crate) fn get_content_line_parts(val: &str) -> Result<ContentLineCaptures<'_>, ParseError> {
    // Default property name to RRULE.
    let property_name = get_property_name(val)?.unwrap_or(PropertyName::RRule);
    match property_name {
        // If the line did not contain a property name (i.e. no ':'), then the
        // entire line is interpreted as the value
        PropertyName::RRule if !val.contains(':') => Ok(ContentLineCaptures {
            property_name: PropertyName::RRule,
            parameters: None,
            value: val.into(),
        }),
        property_name => {
            let mut parameters = None;
            if val.starts_with(&format!("{};", property_name)) {
                let only_colon_idx = val.find(':');
                if let Some(only_colon_idx) = only_colon_idx {
                    parameters = Some(&val[property_name.to_string().len() + 1..only_colon_idx]);
                }
            }

            Ok(ContentLineCaptures {
                property_name,
                parameters,
                value: val
                    .split_once(':')
                    .map(|(_name, val)| val)
                    .unwrap_or_default(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_content_line_parts() {
        let tests = [
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z",
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("TZID=America/Everywhere".into()),
                    value: "20120251T023000Z".into(),
                },
            ),
            (
                "DTSTART:20120251T023000Z",
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: None,
                    value: "20120251T023000Z".into(),
                },
            ),
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z",
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("TZID=America/Everywhere".into()),
                    value: "20120251T023000Z".into(),
                },
            ),
            (
                "RDATE:19970714T123000Z",
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: None,
                    value: "19970714T123000Z".into(),
                },
            ),
            (
                "RDATE;VALUE=DATE:19970101,19970120,19970217,19970421",
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: Some("VALUE=DATE".into()),
                    value: "19970101,19970120,19970217,19970421".into(),
                },
            ),
            (
                "RRULE:FREQ=DAILY;COUNT=10",
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    value: "FREQ=DAILY;COUNT=10".into(),
                },
            ),
            (
                "FREQ=DAILY;COUNT=10",
                ContentLineCaptures {
                    // Defaults to RRULE
                    property_name: PropertyName::RRule,
                    parameters: None,
                    value: "FREQ=DAILY;COUNT=10".into(),
                },
            ),
        ];
        for (input, expected_output) in tests {
            let output = get_content_line_parts(input);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
