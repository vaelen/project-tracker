// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use claude_tracker::{
    config::Config,
    db::{self, Milestone, MilestoneNote, Person, Project, ProjectNote, ProjectStakeholder, StakeholderNote},
};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

// Application state shared across Tauri commands
struct AppState {
    db: Mutex<Connection>,
    config: Config,
}

// Tauri commands (IPC functions callable from frontend)

#[tauri::command]
async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.list_all().map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_project(id: String, state: State<'_, AppState>) -> Result<Option<Project>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.find_by_id(&uuid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_project(project: Project, state: State<'_, AppState>) -> Result<Project, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.create(&project).map_err(|e| e.to_string())?;
    Ok(project)
}

#[tauri::command]
async fn update_project(project: Project, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.update(&project).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.delete(&uuid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_project_milestones(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Milestone>, String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.get_milestones(&uuid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_project_stakeholders(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ProjectStakeholder>, String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.get_stakeholders(&uuid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_milestone(
    milestone: Milestone,
    state: State<'_, AppState>,
) -> Result<Milestone, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.add_milestone(&milestone).map_err(|e| e.to_string())?;
    Ok(milestone)
}

#[tauri::command]
async fn update_milestone(milestone: Milestone, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.update_milestone(&milestone).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_milestone(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.delete_milestone(&uuid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_stakeholder(
    project_id: String,
    stakeholder: ProjectStakeholder,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.add_stakeholder(&uuid, &stakeholder)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_people(state: State<'_, AppState>) -> Result<Vec<Person>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::PersonRepository::new(&db);
    repo.list_all().map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_people(query: String, state: State<'_, AppState>) -> Result<Vec<Person>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::PersonRepository::new(&db);
    repo.search_by_name(&query).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_person(email: String, state: State<'_, AppState>) -> Result<Option<Person>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::PersonRepository::new(&db);
    repo.find_by_email(&email).map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_person(person: Person, state: State<'_, AppState>) -> Result<Person, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::PersonRepository::new(&db);
    repo.create(&person).map_err(|e| e.to_string())?;
    Ok(person)
}

#[tauri::command]
async fn update_person(person: Person, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::PersonRepository::new(&db);
    repo.update(&person).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_person(email: String, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::PersonRepository::new(&db);
    repo.delete(&email).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_jira_url(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.config.jira_url.clone())
}

#[tauri::command]
async fn get_default_email_domain(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.config.default_email_domain.clone())
}

#[tauri::command]
async fn get_project_types(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    Ok(state.config.project_types.clone())
}

#[tauri::command]
async fn chat_with_claude(message: String) -> Result<String, String> {
    // TODO: Implement Claude chat
    Ok(format!("Claude response to: {}", message))
}

// Stakeholder commands

#[tauri::command]
async fn update_stakeholder(
    project_id: String,
    stakeholder: ProjectStakeholder,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.update_stakeholder(&uuid, &stakeholder).map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_stakeholder(
    project_id: String,
    stakeholder_email: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.remove_stakeholder(&uuid, &stakeholder_email).map_err(|e| e.to_string())
}

// Project Note commands

#[tauri::command]
async fn get_project_notes(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ProjectNote>, String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.get_project_notes(&uuid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_note(
    note: ProjectNote,
    state: State<'_, AppState>,
) -> Result<ProjectNote, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.add_project_note(&note).map_err(|e| e.to_string())?;
    Ok(note)
}

#[tauri::command]
async fn update_project_note(
    note: ProjectNote,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.update_project_note(&note).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project_note(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.delete_project_note(&uuid).map_err(|e| e.to_string())
}

// Milestone Note commands

#[tauri::command]
async fn get_milestone_notes(
    milestone_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<MilestoneNote>, String> {
    let uuid = Uuid::parse_str(&milestone_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.get_milestone_notes(&uuid).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_milestone_note(
    note: MilestoneNote,
    state: State<'_, AppState>,
) -> Result<MilestoneNote, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.add_milestone_note(&note).map_err(|e| e.to_string())?;
    Ok(note)
}

#[tauri::command]
async fn update_milestone_note(
    note: MilestoneNote,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.update_milestone_note(&note).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_milestone_note(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.delete_milestone_note(&uuid).map_err(|e| e.to_string())
}

// Stakeholder Note commands

#[tauri::command]
async fn get_stakeholder_notes(
    project_id: String,
    stakeholder_email: String,
    state: State<'_, AppState>,
) -> Result<Vec<StakeholderNote>, String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.get_stakeholder_notes(&uuid, &stakeholder_email).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_stakeholder_note(
    note: StakeholderNote,
    state: State<'_, AppState>,
) -> Result<StakeholderNote, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.add_stakeholder_note(&note).map_err(|e| e.to_string())?;
    Ok(note)
}

#[tauri::command]
async fn update_stakeholder_note(
    note: StakeholderNote,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.update_stakeholder_note(&note).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_stakeholder_note(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let repo = db::ProjectRepository::new(&db);
    repo.delete_stakeholder_note(&uuid).map_err(|e| e.to_string())
}

fn main() {
    env_logger::init();

    // Load configuration
    let config = Config::load_or_default().expect("Failed to load configuration");
    config.ensure_data_dirs().expect("Failed to create data directories");

    // Open database
    let db_path = config.database_path().expect("Failed to get database path");
    let conn = db::open_database(&db_path).expect("Failed to open database");

    // Initialize app state
    let app_state = AppState {
        db: Mutex::new(conn),
        config,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            list_projects,
            get_project,
            create_project,
            update_project,
            delete_project,
            get_project_milestones,
            get_project_stakeholders,
            add_project_milestone,
            update_milestone,
            delete_milestone,
            add_project_stakeholder,
            update_stakeholder,
            remove_stakeholder,
            get_project_notes,
            add_project_note,
            update_project_note,
            delete_project_note,
            get_milestone_notes,
            add_milestone_note,
            update_milestone_note,
            delete_milestone_note,
            get_stakeholder_notes,
            add_stakeholder_note,
            update_stakeholder_note,
            delete_stakeholder_note,
            list_people,
            search_people,
            get_person,
            create_person,
            update_person,
            delete_person,
            get_jira_url,
            get_default_email_domain,
            get_project_types,
            chat_with_claude,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
