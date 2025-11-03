/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { invoke } from '@tauri-apps/api/core';
import type { Project, Milestone, ProjectStakeholder, Person } from '../types';

export class ProjectService {
  /**
   * List all projects
   */
  static async listProjects(): Promise<Project[]> {
    return await invoke<Project[]>('list_projects');
  }

  /**
   * Get a single project by ID
   */
  static async getProject(id: string): Promise<Project | null> {
    return await invoke<Project | null>('get_project', { id });
  }

  /**
   * Create a new project
   */
  static async createProject(project: Project): Promise<Project> {
    return await invoke<Project>('create_project', { project });
  }

  /**
   * Update an existing project
   */
  static async updateProject(project: Project): Promise<void> {
    await invoke('update_project', { project });
  }

  /**
   * Delete a project (cascades to milestones and stakeholders)
   */
  static async deleteProject(id: string): Promise<void> {
    await invoke('delete_project', { id });
  }

  /**
   * Get milestones for a project
   */
  static async getProjectMilestones(projectId: string): Promise<Milestone[]> {
    return await invoke<Milestone[]>('get_project_milestones', { projectId });
  }

  /**
   * Get stakeholders for a project
   */
  static async getProjectStakeholders(projectId: string): Promise<ProjectStakeholder[]> {
    return await invoke<ProjectStakeholder[]>('get_project_stakeholders', { projectId });
  }

  /**
   * Add a milestone to a project
   */
  static async addProjectMilestone(milestone: Milestone): Promise<void> {
    await invoke('add_project_milestone', { milestone });
  }

  /**
   * Add a stakeholder to a project
   */
  static async addProjectStakeholder(projectId: string, stakeholder: ProjectStakeholder): Promise<void> {
    await invoke('add_project_stakeholder', { projectId, stakeholder });
  }

  /**
   * Update a stakeholder
   */
  static async updateStakeholder(projectId: string, stakeholder: ProjectStakeholder): Promise<void> {
    await invoke('update_stakeholder', { projectId, stakeholder });
  }

  /**
   * Remove a stakeholder from a project
   */
  static async removeStakeholder(projectId: string, stakeholderEmail: string): Promise<void> {
    await invoke('remove_stakeholder', { projectId, stakeholderEmail });
  }

  /**
   * List all people
   */
  static async listPeople(): Promise<Person[]> {
    return await invoke<Person[]>('list_people');
  }

  /**
   * Search people by name (for autocomplete)
   */
  static async searchPeople(query: string): Promise<Person[]> {
    return await invoke<Person[]>('search_people', { query });
  }

  /**
   * Get the configured Jira base URL
   */
  static async getJiraUrl(): Promise<string> {
    return await invoke<string>('get_jira_url');
  }

  /**
   * Get the default email domain
   */
  static async getDefaultEmailDomain(): Promise<string> {
    return await invoke<string>('get_default_email_domain');
  }

  /**
   * Construct a full Jira ticket URL from a ticket number
   */
  static async getFullJiraUrl(ticketNumber: string): Promise<string> {
    const baseUrl = await this.getJiraUrl();
    return `${baseUrl}${ticketNumber}`;
  }
}
