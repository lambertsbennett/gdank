use std::fmt::{Display, Formatter};

pub struct Task {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub claude_comments: Option<Vec<String>>,
}

impl Task {
    pub fn new(title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            title,
            content,
            tags,
            claude_comments: None,
        }
    }

    pub fn add_claude_comment(&mut self, comment: String) {
        if let Some(comments) = &mut self.claude_comments {
            comments.push(comment);
        } else {
            self.claude_comments = Some(vec![comment]);
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task: {}\nContent: {}\nTags: {:?}", self.title, self.content, self.tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task_with_no_tags() {
        let task = Task::new("Task Title".to_string(), "Task Content".to_string(), None);
        assert_eq!(task.title, "Task Title");
        assert_eq!(task.content, "Task Content");
        assert!(task.tags.is_none());
        assert!(task.claude_comments.is_none());
    }

    #[test]
    fn create_task_with_tags() {
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let task = Task::new("Task Title".to_string(), "Task Content".to_string(), Some(tags.clone()));
        assert_eq!(task.title, "Task Title");
        assert_eq!(task.content, "Task Content");
        assert_eq!(task.tags, Some(tags));
        assert!(task.claude_comments.is_none());
    }   
}