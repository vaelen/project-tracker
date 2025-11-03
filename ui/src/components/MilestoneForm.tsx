/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Form, Input, Button, Card, message, DatePicker, Space, InputNumber } from 'antd';
import { SaveOutlined, CloseOutlined } from '@ant-design/icons';
import { MilestoneService } from '../services/milestoneService';
import { PersonSelector } from './PersonSelector';
import type { Milestone } from '../types';
import dayjs from 'dayjs';

const { TextArea } = Input;

interface MilestoneFormProps {
  projectId: string;
  milestone?: Milestone;
  onSave: () => void;
  onCancel: () => void;
  isModal?: boolean;
}

export const MilestoneForm: React.FC<MilestoneFormProps> = ({
  projectId,
  milestone,
  onSave,
  onCancel,
  isModal = false,
}) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const isEditing = !!milestone;

  useEffect(() => {
    if (milestone) {
      form.setFieldsValue({
        ...milestone,
        due_date: milestone.due_date ? dayjs(milestone.due_date) : null,
      });
    }
  }, [milestone]);

  const handleSubmit = async (values: any) => {
    setLoading(true);
    try {
      const milestoneData: Milestone = {
        id: milestone?.id || crypto.randomUUID(),
        project_id: projectId,
        number: values.number,
        name: values.name,
        description: values.description || undefined,
        technical_lead: values.technical_lead || undefined,
        design_doc_url: values.design_doc_url || undefined,
        due_date: values.due_date ? values.due_date.toISOString() : undefined,
        jira_epic: values.jira_epic || undefined,
        created_at: milestone?.created_at || new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      if (isEditing) {
        await MilestoneService.updateMilestone(milestoneData);
        message.success('Milestone updated successfully');
      } else {
        await MilestoneService.addMilestone(milestoneData);
        message.success('Milestone created successfully');
      }

      onSave();
    } catch (error) {
      message.error(`Failed to ${isEditing ? 'update' : 'create'} milestone: ` + error);
    } finally {
      setLoading(false);
    }
  };

  const formContent = (
    <Form
      form={form}
      layout="vertical"
      onFinish={handleSubmit}
      initialValues={{
        number: 1,
        name: '',
        description: '',
        technical_lead: undefined,
        design_doc_url: '',
        due_date: null,
        jira_epic: '',
      }}
    >
      <Form.Item
        name="number"
        label="Milestone Number"
        rules={[{ required: true, message: 'Please enter a milestone number' }]}
      >
        <InputNumber min={1} style={{ width: '100%' }} placeholder="Enter milestone number" />
      </Form.Item>

      <Form.Item
        name="name"
        label="Milestone Name"
        rules={[{ required: true, message: 'Please enter a milestone name' }]}
      >
        <Input placeholder="Enter milestone name" />
      </Form.Item>

      <Form.Item
        name="description"
        label="Description"
      >
        <TextArea rows={4} placeholder="Enter milestone description" />
      </Form.Item>

      <Form.Item
        name="technical_lead"
        label="Technical Lead"
      >
        <PersonSelector placeholder="Select technical lead" />
      </Form.Item>

      <Form.Item
        name="design_doc_url"
        label="Design Document URL"
      >
        <Input placeholder="https://..." />
      </Form.Item>

      <Form.Item
        name="due_date"
        label="Due Date"
      >
        <DatePicker style={{ width: '100%' }} />
      </Form.Item>

      <Form.Item
        name="jira_epic"
        label="Jira Epic"
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
            {isEditing ? 'Update' : 'Create'} Milestone
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
    <Card title={isEditing ? 'Edit Milestone' : 'New Milestone'}>
      {formContent}
    </Card>
  );
};
