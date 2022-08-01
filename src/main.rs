#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct EntryBase {
    pub created: String,
    pub modified: String,
    #[serde(rename = "parentfolderid")]
    pub parent_folder_id: Option<u64>,
    pub icon: String,
    pub id: String,
    pub name: String,
    pub path: Option<String>,
    pub thumb: bool,
    #[serde(rename = "isshared")]
    pub is_shared: bool,
    #[serde(rename = "ismine")]
    pub is_mine: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct File {
    #[serde(flatten)]
    pub base: EntryBase,
    #[serde(rename = "fileid")]
    pub file_id: u64,
    pub size: Option<usize>,
    pub hash: Option<usize>,
    #[serde(rename = "contenttype")]
    pub content_type: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Folder {
    #[serde(flatten)]
    pub base: EntryBase,
    #[serde(rename = "folderid")]
    pub folder_id: u64,
    pub contents: Option<Vec<Entry>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Entry {
    File(File),
    Folder(Folder),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct Payload {
    result: u64,
    metadata: Entry,
}

fn main() {
    let payload = include_str!("./payload.json");
    let result: Payload = serde_json::from_str(payload).unwrap();
    println!("result: {:#?}", result);
}
