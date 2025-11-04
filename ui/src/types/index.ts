/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

export interface Person {
  email: string;
  name: string;
  team?: string;
  manager?: string;
  notes?: string;
  created_at: string;
  updated_at: string;
}

export interface Team {
  name: string;
  description?: string;
  manager?: string;
  created_at: string;
  updated_at: string;
}

export interface Project {
  id: string;
  name: string;
  description?: string;
  type: string;
  requirements_owner?: string;
  technical_lead?: string;
  manager?: string;
  team?: string;
  due_date?: string;
  jira_initiative?: string;
  created_at: string;
  updated_at: string;
}

export interface Note {
  id: string;
  title: string;
  body: string;
  created_at: string;
  updated_at: string;
}

export interface ProjectNote extends Note {
  project_id: string;
}

export interface MilestoneNote extends Note {
  milestone_id: string;
}

export interface StakeholderNote extends Note {
  project_id: string;
  stakeholder_email: string;
}

export interface Milestone {
  id: string;
  project_id: string;
  number: number;
  name: string;
  description?: string;
  technical_lead?: string;
  team?: string;
  design_doc_url?: string;
  due_date?: string;
  jira_epic?: string;
  created_at: string;
  updated_at: string;
}

export interface ProjectStakeholder {
  project_id: string;
  stakeholder_email: string;
  role?: string;
  created_at: string;
}
