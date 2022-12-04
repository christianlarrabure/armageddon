use std::io::Write;
use std::{io::Read, path::Path};

use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Topic {
    pub name: String,
    pub sdesc: Option<String>,
}

impl Topic {
    pub fn new(name: String, sdesc: String) -> Self {
        Topic {
            name,
            sdesc: Some(sdesc),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicController {
    pub topics: Vec<Topic>,
}

impl TopicController {
    pub fn new() -> Result<TopicController, String> {
        let path = Path::new("data/topics.json");
        let file = File::open(path);
        if file.is_err() {
            return Err(String::from("Unable to open topics file."));
        }

        let mut file = file.unwrap();
        let mut contents: Vec<u8> = vec![];

        let _ = file.read_to_end(&mut contents);

        let contents = String::from_utf8_lossy(&contents);
        let topics_controller = serde_json::from_str::<TopicController>(&contents);
        let topics_controller = topics_controller.unwrap();
        Ok(topics_controller)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Path::new("data/topics.json");
        let file = File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(path);
        if file.is_err() {
            let _ = file.map_err(|error| println!("Error saving Topics file: {}", error));
            return Err(String::from("Unable to open topics file."));
        }

        let mut file = file.unwrap();

        let contents = serde_json::to_string(self).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        Ok(())
    }

    pub fn add_topic(&self, topic: Topic) -> Self {
        let mut topics = self.topics.clone();
        topics.push(topic);
        TopicController { topics }
    }
}
