#[derive(serde::Deserialize, Debug)]
pub struct ProjectInfo {
    pub name: String,

    #[serde(default)]
    pub freestanding: bool,
}
