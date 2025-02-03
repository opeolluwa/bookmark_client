use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
pub enum Platform {
    Android,
    Ios,
    #[default]
    Mac,
    Linux,
    Windows,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_value = match self {
            Platform::Android => "android".to_string(),
            Platform::Ios => "ios".to_string(),
            Platform::Linux => "linux".to_string(),
            Platform::Windows => "windows".to_string(),
            Platform::Mac => "mac".to_string(),
        };
        write!(f, "{}", string_value)
    }
}
#[derive(Debug, PartialEq)]
pub struct PlatformParseError(String);

impl FromStr for Platform {
    type Err = PlatformParseError;

    fn from_str(platform: &str) -> Result<Self, Self::Err> {
        match platform.to_lowercase().trim() {
            "android" => Ok(Platform::Android),
            "ios" => Ok(Platform::Ios),
            "linux" => Ok(Platform::Linux),
            "windows" => Ok(Platform::Windows),
            "mac" => Ok(Platform::Mac),
            _ => Err(PlatformParseError("Error parsing the platform".to_string())),
        }
    }
}

#[cfg(test)]

mod test {
    use std::str::FromStr;

    use crate::platform::Platform;

    #[test]
    fn platform_from_string() {
        assert_eq!(Platform::from_str("android"), Ok(Platform::Android))
    }

    #[test]
    fn platform_to_string() {
        assert_eq!(Platform::Android.to_string(), String::from("android"))
    }
}
