use std::fmt::Debug;

use parse_display::FromStr;
use serde::Serialize;

#[derive(Debug, FromStr)]
pub enum Formats {
    Debug,
    PrettyDebug,
    Json,
    Yaml,
}

pub fn to_string<O>(format: Formats, output_data: O) -> String
where
    O: Debug + Serialize,
{
    return match format {
        Formats::Debug => {
            format!("{:?}", output_data)
        }
        Formats::PrettyDebug => {
            format!("{:#?}", output_data)
        }
        Formats::Json => serde_json::to_string(&output_data).unwrap(),
        Formats::Yaml => serde_yaml::to_string(&output_data).unwrap(),
    };
}
