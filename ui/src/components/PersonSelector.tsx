/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Select, Modal, message } from 'antd';
import { PersonForm } from './PersonForm';
import { PersonService } from '../services/personService';
import type { Person } from '../types';

interface PersonSelectorProps {
  value?: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  allowClear?: boolean;
}

export const PersonSelector: React.FC<PersonSelectorProps> = ({
  value,
  onChange,
  placeholder = 'Select a person',
  allowClear = true,
}) => {
  const [people, setPeople] = useState<Person[]>([]);
  const [searchValue, setSearchValue] = useState('');
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newPersonName, setNewPersonName] = useState('');

  useEffect(() => {
    loadPeople();
  }, []);

  const loadPeople = async () => {
    try {
      const data = await PersonService.listPeople();
      setPeople(data);
    } catch (error) {
      console.error('Failed to load people:', error);
    }
  };

  const handleSearch = (val: string) => {
    setSearchValue(val);
  };

  const handleSelect = (selectedValue: string) => {
    // If it's a real email (exists in people), use it
    const existingPerson = people.find(p => p.email === selectedValue);
    if (existingPerson) {
      onChange?.(selectedValue);
      setSearchValue('');
      return;
    }

    // Otherwise, it might be a newly typed value
    // Check if it looks like they're trying to add someone new
    if (selectedValue && !selectedValue.includes('@')) {
      // They typed a name, not an email
      setNewPersonName(selectedValue);
      setShowCreateModal(true);
    } else {
      onChange?.(selectedValue);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLElement>) => {
    // If Enter is pressed and the search value doesn't match any existing person
    if (e.key === 'Enter' && searchValue) {
      const matchingPerson = people.find(
        p => p.email === searchValue ||
             p.name.toLowerCase().includes(searchValue.toLowerCase())
      );

      if (!matchingPerson) {
        e.preventDefault();
        e.stopPropagation();
        setNewPersonName(searchValue);
        setShowCreateModal(true);
      }
    }
  };

  const handleCreatePerson = async (person: Person) => {
    try {
      await loadPeople(); // Reload people list
      onChange?.(person.email); // Set the newly created person as selected
      setShowCreateModal(false);
      setSearchValue('');
      message.success(`Created person: ${person.name}`);
    } catch (error) {
      message.error('Failed to create person: ' + error);
    }
  };

  const handleCancelCreate = () => {
    setShowCreateModal(false);
    setNewPersonName('');
    setSearchValue('');
  };

  const options = people.map(person => ({
    label: `${person.name} (${person.email})`,
    value: person.email,
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
        title="Create New Person"
        open={showCreateModal}
        onCancel={handleCancelCreate}
        footer={null}
        width={600}
        destroyOnClose
      >
        <PersonForm
          person={{
            email: '',
            name: newPersonName,
            team: undefined,
            manager: undefined,
            notes: undefined,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
          }}
          onSave={handleCreatePerson}
          onCancel={handleCancelCreate}
          isModal
        />
      </Modal>
    </>
  );
};
