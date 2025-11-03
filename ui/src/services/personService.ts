/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { invoke } from '@tauri-apps/api/core';
import type { Person } from '../types';

export class PersonService {
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
   * Get a single person by email
   */
  static async getPerson(email: string): Promise<Person | null> {
    return await invoke<Person | null>('get_person', { email });
  }

  /**
   * Create a new person
   */
  static async createPerson(person: Person): Promise<Person> {
    return await invoke<Person>('create_person', { person });
  }

  /**
   * Update an existing person
   */
  static async updatePerson(person: Person): Promise<void> {
    await invoke('update_person', { person });
  }

  /**
   * Delete a person
   */
  static async deletePerson(email: string): Promise<void> {
    await invoke('delete_person', { email });
  }

  /**
   * Get the default email domain
   */
  static async getDefaultEmailDomain(): Promise<string> {
    return await invoke<string>('get_default_email_domain');
  }
}
