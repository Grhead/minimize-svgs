pub struct Taskie_projects_collection {
    pub id: String,
    pub title: String,
    pub shortDescription: Option<String>,
    pub logo: Option<String>,
    pub nova: String,
    pub status: Option<Taskie_statuses_collection>,
    pub users: Option<Taskie_users_collection>
}
pub struct Taskie_backlogs_collection {
    pub id: String,
    pub title: String,
    pub nova: String,
    pub tasks: Option<Taskie_tasks_collection>,
    pub tags: Option<Taskie_tags_collection>,
    pub groups: Option<Taskie_backlog_groups_collection>
}
pub struct Taskie_backlog_groups_collection {
    pub id: String,
    pub title: String,
    pub nova: String,
}
pub struct Taskie_roles_collection {
    pub id: String,
    pub title: String,
}
pub struct Taskie_users_collection {
    pub id: String,
    pub login: String,
    pub nova: String,
    pub roles: Option<Taskie_roles_collection>
}
pub struct Taskie_levels_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}

pub struct Taskie_tags_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
pub struct Taskie_statuses_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
pub struct Taskie_priorities_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
pub struct Taskie_modules_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
pub struct Taskie_files_collection {
    pub id: String,
    pub title: String,
    pub path: String
}
pub struct Taskie_completness_collection {
    pub id: String,
    pub title: String,
    pub nova: String
}
pub struct Taskie_tasks_collection {
    pub id: String,
    pub title: String,
    pub description: String,
    pub nova: String,
    pub files: Option<Taskie_files_collection>,
    pub priority: Option<Taskie_priorities_collection>,
    pub levels: Option<Taskie_levels_collection>,
    pub tags: Option<Taskie_tags_collection>,
    pub modules: Option<Taskie_modules_collection>,
    pub completeness: Option<Taskie_completness_collection>,
    pub responsiblePersons: Option<Taskie_users_collection>,
}
pub struct Taskie_edge_collection {}