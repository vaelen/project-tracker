/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Form, Input, Button, Card, message, DatePicker, Space, Select } from 'antd';
import { SaveOutlined, CloseOutlined } from '@ant-design/icons';
import { ProjectService } from '../services/projectService';
import { PersonSelector } from './PersonSelector';
import { TeamSelector } from './TeamSelector';
import type { Project } from '../types';
import dayjs from 'dayjs';

const { TextArea } = Input;

interface ProjectFormProps {
  project?: Project;
  onSave: () => void;
  onCancel: () => void;
}

export const ProjectForm: React.FC<ProjectFormProps> = ({ project, onSave, onCancel }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const isEditing = !!project;

  useEffect(() => {
    if (project) {
      form.setFieldsValue({
        ...project,
        start_date: project.start_date ? dayjs(project.start_date) : null,
        due_date: project.due_date ? dayjs(project.due_date) : null,
      });
    }
  }, [project]);

  const handleSubmit = async (values: any) => {
    setLoading(true);
    try {
      const projectData: Project = {
        id: project?.id || crypto.randomUUID(),
        name: values.name,
        description: values.description || undefined,
        type: values.type || 'Personal',
        requirements_owner: values.requirements_owner || undefined,
        technical_lead: values.technical_lead || undefined,
        manager: values.manager || undefined,
        team: values.team || undefined,
        start_date: values.start_date ? values.start_date.toISOString() : undefined,
        due_date: values.due_date ? values.due_date.toISOString() : undefined,
        jira_initiative: values.jira_initiative || undefined,
        created_at: project?.created_at || new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      if (isEditing) {
        await ProjectService.updateProject(projectData);
        message.success('Project updated successfully');
      } else {
        await ProjectService.createProject(projectData);
        message.success('Project created successfully');
      }

      onSave();
    } catch (error) {
      message.error(`Failed to ${isEditing ? 'update' : 'create'} project: ` + error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card title={isEditing ? 'Edit Project' : 'New Project'}>
      <Form
        form={form}
        layout="vertical"
        onFinish={handleSubmit}
        initialValues={{
          name: '',
          description: '',
          type: 'Personal',
          requirements_owner: undefined,
          technical_lead: undefined,
          manager: undefined,
          team: undefined,
          start_date: null,
          due_date: null,
          jira_initiative: '',
        }}
      >
        <Form.Item
          name="name"
          label="Project Name"
          rules={[{ required: true, message: 'Please enter a project name' }]}
        >
          <Input placeholder="Enter project name" />
        </Form.Item>

        <Form.Item
          name="description"
          label="Description"
        >
          <TextArea rows={4} placeholder="Enter project description" />
        </Form.Item>

        <Form.Item
          name="type"
          label="Project Type"
          rules={[{ required: true, message: 'Please select a project type' }]}
        >
          <Select placeholder="Select project type">
            <Select.Option value="Personal">Personal</Select.Option>
            <Select.Option value="Team">Team</Select.Option>
            <Select.Option value="Company">Company</Select.Option>
          </Select>
        </Form.Item>

        <Form.Item
          name="requirements_owner"
          label="Requirements Owner"
        >
          <PersonSelector placeholder="Select requirements owner" />
        </Form.Item>

        <Form.Item
          name="technical_lead"
          label="Technical Lead"
        >
          <PersonSelector placeholder="Select technical lead" />
        </Form.Item>

        <Form.Item
          name="manager"
          label="Manager"
        >
          <PersonSelector placeholder="Select manager" />
        </Form.Item>

        <Form.Item
          name="team"
          label="Team"
        >
          <TeamSelector placeholder="Select team" />
        </Form.Item>

        <Form.Item
          name="start_date"
          label="Start Date"
        >
          <DatePicker style={{ width: '100%' }} />
        </Form.Item>

        <Form.Item
          name="due_date"
          label="Due Date"
        >
          <DatePicker style={{ width: '100%' }} />
        </Form.Item>

        <Form.Item
          name="jira_initiative"
          label="Jira Initiative"
          help="Enter only the ticket number (e.g., PROJ-123)"
        >
          <Input placeholder="e.g., PROJ-123" />
        </Form.Item>

        <Form.Item>
          <Space>
            <Button
              type="primary"
              htmlType="submit"
              icon={<SaveOutlined />}
              loading={loading}
            >
              {isEditing ? 'Update' : 'Create'} Project
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
    </Card>
  );
};
