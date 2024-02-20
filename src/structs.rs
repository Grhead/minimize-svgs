use aragog::Record;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_projects_collection {
    pub id: String,
    pub title: String,
    pub shortDescription: String,
    pub logo: String,
    pub nova: String
    // status (statuses)
    // owner (users)
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_backlogs_collection {
    pub id: String,
    pub title: String,
    pub nova: String
    // tasks
    // tags
    // groups (sections)
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_backlog_groups_collection {
    pub id: String,
    pub title: String,
    pub nova: String,
}

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_roles_collection {
    pub id: String,
    pub title: String,
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_files_collection {
    pub id: String,
    pub title: String,
    pub path: String,
    pub nova: String
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_users_collection {
    pub id: String,
    pub login: String,
    pub nova: String
    // roles
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_levels_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_tags_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_statuses_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_priorities_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_modules_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_completness_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_tasks_collection {
    pub id: String,
    pub title: String,
    pub description: String,
    pub nova: String
    // files
    // priority
    // levels
    // tags
    // modules
    // completeness
    // responsiblePersons

}
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Taskie_edge_collection {}