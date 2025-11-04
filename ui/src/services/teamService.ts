/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { invoke } from '@tauri-apps/api/core';
import type { Team, Person } from '../types';

export class TeamService {
  /**
   * List all teams
   */
  static async listTeams(): Promise<Team[]> {
    return await invoke<Team[]>('list_teams');
  }

  /**
   * Search teams by name (for autocomplete)
   */
  static async searchTeams(query: string): Promise<Team[]> {
    return await invoke<Team[]>('search_teams', { query });
  }

  /**
   * Get a single team by name
   */
  static async getTeam(name: string): Promise<Team | null> {
    return await invoke<Team | null>('get_team', { name });
  }

  /**
   * Create a new team
   */
  static async createTeam(team: Team): Promise<Team> {
    return await invoke<Team>('create_team', { team });
  }

  /**
   * Update an existing team
   */
  static async updateTeam(team: Team): Promise<void> {
    await invoke('update_team', { team });
  }

  /**
   * Delete a team
   */
  static async deleteTeam(name: string): Promise<void> {
    await invoke('delete_team', { name });
  }

  /**
   * Add a member to a team
   */
  static async addTeamMember(teamName: string, personEmail: string): Promise<void> {
    await invoke('add_team_member', { teamName, personEmail });
  }

  /**
   * Remove a member from a team
   */
  static async removeTeamMember(teamName: string, personEmail: string): Promise<void> {
    await invoke('remove_team_member', { teamName, personEmail });
  }

  /**
   * Get all members of a team
   */
  static async getTeamMembers(teamName: string): Promise<Person[]> {
    return await invoke<Person[]>('get_team_members', { teamName });
  }
}
