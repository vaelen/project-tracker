/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { invoke } from '@tauri-apps/api/core';
import type { Milestone } from '../types';

export class MilestoneService {
  static async addMilestone(milestone: Milestone): Promise<Milestone> {
    return await invoke<Milestone>('add_project_milestone', { milestone });
  }

  static async updateMilestone(milestone: Milestone): Promise<void> {
    await invoke('update_milestone', { milestone });
  }

  static async deleteMilestone(id: string): Promise<void> {
    await invoke('delete_milestone', { id });
  }
}
