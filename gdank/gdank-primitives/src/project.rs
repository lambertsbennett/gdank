use crate::note::Note;
use crate::task::Task;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Vec<Note>>,
    pub tasks: Option<Vec<Task>>,
}

impl Project {
    pub fn new(name: String, description: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            tags,
            notes: None,
            tasks: None,
        }
    }

    pub fn add_note(&mut self, note: Note) {
        if let Some(notes) = &mut self.notes {
            notes.push(note);
        } else {
            self.notes = Some(vec![note]);
        }
    }

    pub fn add_task(&mut self, task: Task) {
        if let Some(tasks) = &mut self.tasks {
            tasks.push(task);
        } else {
            self.tasks = Some(vec![task]);
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Project: {}\nDescription: {}\nTags: {:?}", self.name, self.description, self.tags)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_project_with_no_tags() {
        let project = Project::new("Project Name".to_string(), "Project Description".to_string(), None);
        assert!(!project.id.is_empty());
        assert_eq!(project.name, "Project Name");
        assert_eq!(project.description, "Project Description");
        assert!(project.tags.is_none());
        assert!(project.notes.is_none());
    }

    #[test]
    fn create_project_with_tags() {
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let project = Project::new("Project Name".to_string(), "Project Description".to_string(), Some(tags.clone()));
        assert!(!project.id.is_empty());
        assert_eq!(project.name, "Project Name");
        assert_eq!(project.description, "Project Description");
        assert_eq!(project.tags, Some(tags));
        assert!(project.notes.is_none());
    }

    #[test]
    fn add_note_to_project() {
        let mut project = Project::new("Project Name".to_string(), "Project Description".to_string(), None);
        let note = Note::new("Note Title".to_string(), "Note Content".to_string());
        project.add_note(note);
        assert!(project.notes.is_some());
        assert_eq!(project.notes.as_ref().unwrap().len(), 1);
        assert_eq!(project.notes.as_ref().unwrap()[0].title, "Note Title");
        assert_eq!(project.notes.as_ref().unwrap()[0].content, "Note Content");
    }

    #[test]
    fn add_task_to_project() {
        let mut project = Project::new("Project Name".to_string(), "Project Description".to_string(), None);
        let task = Task::new("Task Title".to_string(), "Task Content".to_string(), None);
        project.add_task(task);
        assert!(project.tasks.is_some());
        assert_eq!(project.tasks.as_ref().unwrap().len(), 1);
        assert_eq!(project.tasks.as_ref().unwrap()[0].title, "Task Title");
        assert_eq!(project.tasks.as_ref().unwrap()[0].content, "Task Content");
        assert!(project.tasks.as_ref().unwrap()[0].tags.is_none());
    }
}