use gdank_primitives::project::Project;

pub enum CurrentScreen {
    Overview,
    Projects,
    Tasks,
    Notes
}

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub projects: Vec<Project>, // a list of projects
}

impl App {
    pub fn new() -> App {
        let mut projects = Vec::<Project>::new();
        projects.push(Project::new("Project 1".to_string(), "This is the first project".to_string(), None));
        projects.push(Project::new("Project 2".to_string(), "This is the second project".to_string(), None));

        App {
            current_screen: CurrentScreen::Overview,
            projects: projects,

        }
    }
}