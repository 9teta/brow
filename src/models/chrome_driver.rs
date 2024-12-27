use serde::Deserialize;
use serde::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeDriverRoot {
    pub timestamp: String,
    pub channels: Channels,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channels {
    #[serde(rename = "Stable")]
    pub stable: Stable,
    #[serde(rename = "Beta")]
    pub beta: Beta,
    #[serde(rename = "Dev")]
    pub dev: Dev,
    #[serde(rename = "Canary")]
    pub canary: Canary,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stable {
    pub channel: String,
    pub version: String,
    pub revision: String,
    pub downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub chrome: Vec<Chrome>,
    pub chromedriver: Vec<Chromedriver>,
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<ChromeHeadlessShell>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chrome {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chromedriver {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeHeadlessShell {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beta {
    pub channel: String,
    pub version: String,
    pub revision: String,
    pub downloads: Downloads2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads2 {
    pub chrome: Vec<Chrome2>,
    pub chromedriver: Vec<Chromedriver2>,
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<ChromeHeadlessShell2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chrome2 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chromedriver2 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeHeadlessShell2 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dev {
    pub channel: String,
    pub version: String,
    pub revision: String,
    pub downloads: Downloads3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads3 {
    pub chrome: Vec<Chrome3>,
    pub chromedriver: Vec<Chromedriver3>,
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<ChromeHeadlessShell3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chrome3 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chromedriver3 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeHeadlessShell3 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canary {
    pub channel: String,
    pub version: String,
    pub revision: String,
    pub downloads: Downloads4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads4 {
    pub chrome: Vec<Chrome4>,
    pub chromedriver: Vec<Chromedriver4>,
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<ChromeHeadlessShell4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chrome4 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chromedriver4 {
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChromeHeadlessShell4 {
    pub platform: String,
    pub url: String,
}
