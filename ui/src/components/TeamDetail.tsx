/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Card, Descriptions, Button, Space, Table, message, Modal, Select } from 'antd';
import { EditOutlined, ArrowLeftOutlined, UserAddOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { TeamService } from '../services/teamService';
import { PersonService } from '../services/personService';
import { invoke } from '@tauri-apps/api/core';
import type { Team, Person, Project } from '../types';

interface TeamDetailProps {
  team: Team;
  onEdit: () => void;
  onBack: () => void;
  onViewProject?: (project: Project) => void;
}

export const TeamDetail: React.FC<TeamDetailProps> = ({ team, onEdit, onBack, onViewProject }) => {
  const [members, setMembers] = useState<Person[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [availablePeople, setAvailablePeople] = useState<Person[]>([]);
  const [loading, setLoading] = useState(false);
  const [addMemberModalVisible, setAddMemberModalVisible] = useState(false);
  const [selectedPersonEmail, setSelectedPersonEmail] = useState<string | undefined>();

  useEffect(() => {
    loadMembers();
    loadProjects();
    loadAvailablePeople();
  }, [team.name]);

  const loadMembers = async () => {
    try {
      const data = await TeamService.getTeamMembers(team.name);
      setMembers(data);
    } catch (error) {
      message.error('Failed to load team members: ' + error);
    }
  };

  const loadProjects = async () => {
    try {
      const allProjects = await invoke<Project[]>('list_projects');
      // Filter projects assigned to this team
      const teamProjects = allProjects.filter(p => p.team === team.name);
      setProjects(teamProjects);
    } catch (error) {
      message.error('Failed to load projects: ' + error);
    }
  };

  const loadAvailablePeople = async () => {
    try {
      const allPeople = await PersonService.listPeople();
      setAvailablePeople(allPeople);
    } catch (error) {
      message.error('Failed to load people: ' + error);
    }
  };

  const handleAddMember = async () => {
    if (!selectedPersonEmail) {
      message.warning('Please select a person to add');
      return;
    }

    setLoading(true);
    try {
      await TeamService.addTeamMember(team.name, selectedPersonEmail);
      message.success('Member added successfully');
      setAddMemberModalVisible(false);
      setSelectedPersonEmail(undefined);
      loadMembers();
    } catch (error) {
      message.error('Failed to add member: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleRemoveMember = async (personEmail: string) => {
    Modal.confirm({
      title: 'Remove Team Member',
      content: 'Are you sure you want to remove this member from the team?',
      okText: 'Remove',
      okType: 'danger',
      onOk: async () => {
        try {
          await TeamService.removeTeamMember(team.name, personEmail);
          message.success('Member removed successfully');
          loadMembers();
        } catch (error) {
          message.error('Failed to remove member: ' + error);
        }
      },
    });
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString();
  };

  const memberColumns: ColumnsType<Person> = [
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      sorter: (a, b) => a.name.localeCompare(b.name),
    },
    {
      title: 'Email',
      dataIndex: 'email',
      key: 'email',
    },
    {
      title: 'Manager',
      dataIndex: 'manager',
      key: 'manager',
      render: (text) => text || '-',
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 120,
      render: (_, record) => (
        <Button
          type="link"
          size="small"
          danger
          icon={<DeleteOutlined />}
          onClick={() => handleRemoveMember(record.email)}
        >
          Remove
        </Button>
      ),
    },
  ];

  const projectColumns: ColumnsType<Project> = [
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      sorter: (a, b) => a.name.localeCompare(b.name),
      render: (text, record) => (
        onViewProject ? (
          <Button type="link" onClick={() => onViewProject(record)}>
            {text}
          </Button>
        ) : text
      ),
    },
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
    },
    {
      title: 'Manager',
      dataIndex: 'manager',
      key: 'manager',
      render: (text) => text || '-',
    },
    {
      title: 'Due Date',
      dataIndex: 'due_date',
      key: 'due_date',
      render: formatDate,
    },
  ];

  // Filter available people (exclude current members)
  const memberEmails = new Set(members.map(m => m.email));
  const peopleOptions = availablePeople
    .filter(p => !memberEmails.has(p.email))
    .map(p => ({
      label: `${p.name} (${p.email})`,
      value: p.email,
    }));

  return (
    <div>
      <Space style={{ marginBottom: 16 }}>
        <Button icon={<ArrowLeftOutlined />} onClick={onBack}>
          Back to Teams
        </Button>
        <Button type="primary" icon={<EditOutlined />} onClick={onEdit}>
          Edit Team
        </Button>
      </Space>

      <Card title={`Team: ${team.name}`} style={{ marginBottom: 16 }}>
        <Descriptions column={2} bordered>
          <Descriptions.Item label="Name" span={2}>
            {team.name}
          </Descriptions.Item>
          <Descriptions.Item label="Description" span={2}>
            {team.description || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Manager" span={2}>
            {team.manager || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Created">
            {formatDate(team.created_at)}
          </Descriptions.Item>
          <Descriptions.Item label="Last Updated">
            {formatDate(team.updated_at)}
          </Descriptions.Item>
        </Descriptions>
      </Card>

      <Card
        title="Team Members"
        style={{ marginBottom: 16 }}
        extra={
          <Button
            type="primary"
            size="small"
            icon={<UserAddOutlined />}
            onClick={() => setAddMemberModalVisible(true)}
          >
            Add Member
          </Button>
        }
      >
        <Table
          columns={memberColumns}
          dataSource={members}
          rowKey="email"
          pagination={false}
          locale={{ emptyText: 'No members in this team' }}
        />
      </Card>

      <Card title="Assigned Projects">
        <Table
          columns={projectColumns}
          dataSource={projects}
          rowKey="id"
          pagination={false}
          locale={{ emptyText: 'No projects assigned to this team' }}
        />
      </Card>

      <Modal
        title="Add Team Member"
        open={addMemberModalVisible}
        onOk={handleAddMember}
        onCancel={() => {
          setAddMemberModalVisible(false);
          setSelectedPersonEmail(undefined);
        }}
        confirmLoading={loading}
      >
        <Select
          showSearch
          style={{ width: '100%' }}
          placeholder="Select person to add"
          value={selectedPersonEmail}
          onChange={setSelectedPersonEmail}
          options={peopleOptions}
          filterOption={(input, option) =>
            (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
          }
        />
      </Modal>
    </div>
  );
};
