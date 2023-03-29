use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct Agile {
    pub name: String,
    pub current_sprint: Sprint,
    pub column_settings: ColumnSettings,
    pub estimation_field: Option<CustomField>,
    pub original_estimation_field: Option<CustomField>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct ColumnSettings {
    pub field: CustomField,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct CustomField {
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct Sprint {
    pub issues: Vec<Issue>,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct Issue {
    pub id_readable: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub parent: Option<IssueLink>,
    pub project: Option<Project>,
    pub custom_fields: Vec<IssueCustomField>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct IssueCustomField {
    pub name: String,
    pub value: CustomFieldValue,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
#[serde(tag = "$type")]
pub enum CustomFieldValue {
    #[default]
    Nothing,
    EnumBundleElement,
    StateBundleElement,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct PeriodValue {
    pub minutes: i32,
    pub presentation: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct TextFieldValue {
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct Project {
    pub short_name: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct IssueLink {
    pub issues: Option<Vec<Issue>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
#[serde(default)]
pub struct Tag {
    pub name: String,
}
