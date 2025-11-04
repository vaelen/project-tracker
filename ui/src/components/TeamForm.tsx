/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Form, Input, Button, Card, message, Select, Space } from 'antd';
import { SaveOutlined, CloseOutlined } from '@ant-design/icons';
import { TeamService } from '../services/teamService';
import { PersonService } from '../services/personService';
import type { Team, Person } from '../types';

const { TextArea } = Input;

interface TeamFormProps {
  team?: Team;
  initialName?: string;
  onSave: (team: Team) => void;
  onCancel: () => void;
  isModal?: boolean;
}

export const TeamForm: React.FC<TeamFormProps> = ({ team, initialName, onSave, onCancel, isModal = false }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [people, setPeople] = useState<Person[]>([]);

  const isEditing = !!team && !!team.name;

  useEffect(() => {
    loadPeople();

    if (team) {
      form.setFieldsValue(team);
    } else if (initialName) {
      form.setFieldsValue({ name: initialName });
    }
  }, [team, initialName]);

  const loadPeople = async () => {
    try {
      const data = await PersonService.listPeople();
      setPeople(data);
    } catch (error) {
      message.error('Failed to load people: ' + error);
    }
  };

  const handleSubmit = async (values: any) => {
    setLoading(true);
    try {
      const teamData: Team = {
        name: values.name,
        description: values.description || undefined,
        manager: values.manager || undefined,
        created_at: team?.created_at || new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      if (isEditing) {
        await TeamService.updateTeam(teamData);
        message.success('Team updated successfully');
      } else {
        await TeamService.createTeam(teamData);
        message.success('Team created successfully');
      }

      onSave(teamData);
    } catch (error) {
      message.error(`Failed to ${isEditing ? 'update' : 'create'} team: ` + error);
    } finally {
      setLoading(false);
    }
  };

  const managerOptions = people.map(p => ({
    label: `${p.name} (${p.email})`,
    value: p.email,
  }));

  const formContent = (
    <Form
      form={form}
      layout="vertical"
      onFinish={handleSubmit}
      initialValues={{
        name: '',
        description: '',
        manager: undefined,
      }}
    >
      <Form.Item
        name="name"
        label="Team Name"
        rules={[{ required: true, message: 'Please enter a team name' }]}
      >
        <Input placeholder="Enter team name" disabled={isEditing} />
      </Form.Item>

      <Form.Item
        name="description"
        label="Description"
      >
        <TextArea rows={3} placeholder="Enter team description" />
      </Form.Item>

      <Form.Item
        name="manager"
        label="Manager"
      >
        <Select
          showSearch
          placeholder="Select manager"
          options={managerOptions}
          filterOption={(input, option) =>
            (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
          }
          allowClear
        />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            htmlType="submit"
            icon={<SaveOutlined />}
            loading={loading}
          >
            {isEditing ? 'Update' : 'Create'} Team
          </Button>
          <Button
            icon={<CloseOutlined />}
            onClick={onCancel}
            disabled={loading}
          >
            Cancel
          </Button>
        </Space>
      </Form.Item>
    </Form>
  );

  if (isModal) {
    return formContent;
  }

  return (
    <Card title={isEditing ? 'Edit Team' : 'New Team'}>
      {formContent}
    </Card>
  );
};
