use std::fs::File;
use std::io::Result;
use gdank_primitives::project::Project;
use serde_json::to_writer_pretty;

pub fn write_project_to_file(project: &Project, file_path: &str) -> Result<()> {
    let file = File::create(file_path)?;
    to_writer_pretty(file, project)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_project_to_file() {
        let project = Project::new("Test Project".to_string(), "This is a test project".to_string(), None);
        let file_path = "test_project.json";
        let result = write_project_to_file(&project, file_path);
        assert!(result.is_ok());
    }
}