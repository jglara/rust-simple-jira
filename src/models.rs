use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Status::Open => "OPEN",
            Status::InProgress => "IN PROGRESS",
            Status::Resolved => "RESOLVED",
            Status::Closed => "CLOSED",
        };
        f.write_str(str)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,

    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Self {
            name: name,
            description: description,
            status: Status::Open,
            stories: vec![],
        }
    }

}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name: name,
            description: description,
            status: Status::Open,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DBState {
      // This struct represents the entire db state which includes the last_item_id, epics, and stories
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
  
  
}