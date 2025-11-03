/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { invoke } from '@tauri-apps/api/core';
import type { ProjectNote, MilestoneNote, StakeholderNote } from '../types';

export class NoteService {
  // Project Notes
  static async getProjectNotes(projectId: string): Promise<ProjectNote[]> {
    return await invoke<ProjectNote[]>('get_project_notes', { projectId });
  }

  static async addProjectNote(note: ProjectNote): Promise<ProjectNote> {
    return await invoke<ProjectNote>('add_project_note', { note });
  }

  static async updateProjectNote(note: ProjectNote): Promise<void> {
    await invoke('update_project_note', { note });
  }

  static async deleteProjectNote(id: string): Promise<void> {
    await invoke('delete_project_note', { id });
  }

  // Milestone Notes
  static async getMilestoneNotes(milestoneId: string): Promise<MilestoneNote[]> {
    return await invoke<MilestoneNote[]>('get_milestone_notes', { milestoneId });
  }

  static async addMilestoneNote(note: MilestoneNote): Promise<MilestoneNote> {
    return await invoke<MilestoneNote>('add_milestone_note', { note });
  }

  static async updateMilestoneNote(note: MilestoneNote): Promise<void> {
    await invoke('update_milestone_note', { note });
  }

  static async deleteMilestoneNote(id: string): Promise<void> {
    await invoke('delete_milestone_note', { id });
  }

  // Stakeholder Notes
  static async getStakeholderNotes(projectId: string, stakeholderEmail: string): Promise<StakeholderNote[]> {
    return await invoke<StakeholderNote[]>('get_stakeholder_notes', { projectId, stakeholderEmail });
  }

  static async addStakeholderNote(note: StakeholderNote): Promise<StakeholderNote> {
    return await invoke<StakeholderNote>('add_stakeholder_note', { note });
  }

  static async updateStakeholderNote(note: StakeholderNote): Promise<void> {
    await invoke('update_stakeholder_note', { note });
  }

  static async deleteStakeholderNote(id: string): Promise<void> {
    await invoke('delete_stakeholder_note', { id });
  }
}
