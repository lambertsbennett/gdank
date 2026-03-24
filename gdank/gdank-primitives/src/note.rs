use std::fmt::{Display, Formatter};

pub struct Note {
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Note: {}\nContent: {}", self.title, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_note() {
        let note = Note::new("Note Title".to_string(), "Note Content".to_string());
        assert_eq!(note.title, "Note Title");
        assert_eq!(note.content, "Note Content");
    }
}
