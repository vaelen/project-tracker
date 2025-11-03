/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'react';
import { Form, Input, Button, message, Space } from 'antd';
import { SaveOutlined, CloseOutlined } from '@ant-design/icons';
import { PersonSelector } from './PersonSelector';
import type { ProjectStakeholder } from '../types';

interface StakeholderFormProps {
  projectId: string;
  stakeholder?: ProjectStakeholder;
  onSave: (stakeholder: ProjectStakeholder) => Promise<void>;
  onCancel: () => void;
  isModal?: boolean;
}

export const StakeholderForm: React.FC<StakeholderFormProps> = ({
  projectId,
  stakeholder,
  onSave,
  onCancel,
  isModal = false,
}) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const isEditing = !!stakeholder;

  const handleSubmit = async (values: any) => {
    setLoading(true);
    try {
      const stakeholderData: ProjectStakeholder = {
        project_id: projectId,
        stakeholder_email: values.stakeholder_email,
        role: values.role || undefined,
        created_at: stakeholder?.created_at || new Date().toISOString(),
      };

      await onSave(stakeholderData);
      if (!isModal) {
        form.resetFields();
      }
    } catch (error) {
      message.error(`Failed to ${isEditing ? 'update' : 'add'} stakeholder: ` + error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Form
      form={form}
      layout="vertical"
      onFinish={handleSubmit}
      initialValues={{
        stakeholder_email: stakeholder?.stakeholder_email || undefined,
        role: stakeholder?.role || '',
      }}
    >
      <Form.Item
        name="stakeholder_email"
        label="Stakeholder"
        rules={[{ required: true, message: 'Please select a stakeholder' }]}
      >
        <PersonSelector placeholder="Select stakeholder" />
      </Form.Item>

      <Form.Item
        name="role"
        label="Role"
      >
        <Input placeholder="Enter stakeholder role (e.g., Sponsor, Reviewer)" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            htmlType="submit"
            icon={<SaveOutlined />}
            loading={loading}
          >
            {isEditing ? 'Update' : 'Add'} Stakeholder
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
};
