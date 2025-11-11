/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'react';
import { Form, Input, Button, message, Space } from 'antd';
import { SaveOutlined, CloseOutlined } from '@ant-design/icons';
import { PersonSelector } from './PersonSelector';
import type { MilestoneResource } from '../types';

interface MilestoneResourceFormProps {
  milestoneId: string;
  resource?: MilestoneResource;
  onSave: (resource: MilestoneResource) => Promise<void>;
  onCancel: () => void;
  isModal?: boolean;
}

export const MilestoneResourceForm: React.FC<MilestoneResourceFormProps> = ({
  milestoneId,
  resource,
  onSave,
  onCancel,
  isModal = false,
}) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const isEditing = !!resource;

  const handleSubmit = async (values: any) => {
    setLoading(true);
    try {
      const resourceData: MilestoneResource = {
        milestone_id: milestoneId,
        person_email: values.person_email,
        role: values.role || undefined,
        created_at: resource?.created_at || new Date().toISOString(),
      };

      await onSave(resourceData);
      if (!isModal) {
        form.resetFields();
      }
    } catch (error) {
      message.error(`Failed to ${isEditing ? 'update' : 'add'} resource: ` + error);
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
        person_email: resource?.person_email || undefined,
        role: resource?.role || '',
      }}
    >
      <Form.Item
        name="person_email"
        label="Person"
        rules={[{ required: true, message: 'Please select a person' }]}
      >
        <PersonSelector placeholder="Select person" />
      </Form.Item>

      <Form.Item
        name="role"
        label="Role"
      >
        <Input placeholder="Enter role (e.g., Developer, Designer, PM)" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            htmlType="submit"
            icon={<SaveOutlined />}
            loading={loading}
          >
            {isEditing ? 'Update' : 'Add'} Resource
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
