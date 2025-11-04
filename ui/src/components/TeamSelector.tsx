/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Select, Modal, message } from 'antd';
import { TeamForm } from './TeamForm';
import { TeamService } from '../services/teamService';
import type { Team } from '../types';

interface TeamSelectorProps {
  value?: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  allowClear?: boolean;
}

export const TeamSelector: React.FC<TeamSelectorProps> = ({
  value,
  onChange,
  placeholder = 'Select a team',
  allowClear = true,
}) => {
  const [teams, setTeams] = useState<Team[]>([]);
  const [searchValue, setSearchValue] = useState('');
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newTeamName, setNewTeamName] = useState('');

  useEffect(() => {
    loadTeams();
  }, []);

  const loadTeams = async () => {
    try {
      const data = await TeamService.listTeams();
      setTeams(data);
    } catch (error) {
      console.error('Failed to load teams:', error);
    }
  };

  const handleSearch = (val: string) => {
    setSearchValue(val);
  };

  const handleSelect = (selectedValue: string) => {
    // If it's a real team name (exists in teams), use it
    const existingTeam = teams.find(t => t.name === selectedValue);
    if (existingTeam) {
      onChange?.(selectedValue);
      setSearchValue('');
      return;
    }

    // Otherwise, treat as new team name
    onChange?.(selectedValue);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLElement>) => {
    // If Enter is pressed and the search value doesn't match any existing team
    if (e.key === 'Enter' && searchValue) {
      const matchingTeam = teams.find(
        t => t.name.toLowerCase() === searchValue.toLowerCase()
      );

      if (!matchingTeam) {
        e.preventDefault();
        e.stopPropagation();
        setNewTeamName(searchValue);
        setShowCreateModal(true);
      }
    }
  };

  const handleCreateTeam = async (team: Team) => {
    try {
      await loadTeams(); // Reload teams list
      onChange?.(team.name); // Set the newly created team as selected
      setShowCreateModal(false);
      setSearchValue('');
      message.success(`Created team: ${team.name}`);
    } catch (error) {
      message.error('Failed to create team: ' + error);
    }
  };

  const handleCancelCreate = () => {
    setShowCreateModal(false);
    setNewTeamName('');
    setSearchValue('');
  };

  const options = teams.map(team => ({
    label: team.name,
    value: team.name,
  }));

  return (
    <>
      <Select
        showSearch
        value={value}
        placeholder={placeholder}
        options={options}
        onChange={handleSelect}
        onSearch={handleSearch}
        onInputKeyDown={handleKeyDown}
        filterOption={(input, option) =>
          (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
        }
        allowClear={allowClear}
        notFoundContent={
          searchValue ? (
            <div style={{ padding: '8px', textAlign: 'center' }}>
              No match found. Press Enter to create &quot;{searchValue}&quot;
            </div>
          ) : null
        }
      />

      <Modal
        title="Create New Team"
        open={showCreateModal}
        onCancel={handleCancelCreate}
        footer={null}
        width={600}
        destroyOnClose
      >
        <TeamForm
          initialName={newTeamName}
          onSave={handleCreateTeam}
          onCancel={handleCancelCreate}
          isModal
        />
      </Modal>
    </>
  );
};
