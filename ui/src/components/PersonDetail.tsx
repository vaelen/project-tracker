/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { Card, Descriptions, Button, Space } from 'antd';
import { EditOutlined, ArrowLeftOutlined } from '@ant-design/icons';
import type { Person } from '../types';

interface PersonDetailProps {
  person: Person;
  onEdit: () => void;
  onBack: () => void;
}

export const PersonDetail: React.FC<PersonDetailProps> = ({ person, onEdit, onBack }) => {
  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString();
  };

  return (
    <div>
      <Space style={{ marginBottom: 16 }}>
        <Button icon={<ArrowLeftOutlined />} onClick={onBack}>
          Back to People
        </Button>
        <Button type="primary" icon={<EditOutlined />} onClick={onEdit}>
          Edit Person
        </Button>
      </Space>

      <Card title={`Person: ${person.name}`}>
        <Descriptions column={2} bordered>
          <Descriptions.Item label="Name" span={2}>
            {person.name}
          </Descriptions.Item>
          <Descriptions.Item label="Email" span={2}>
            {person.email}
          </Descriptions.Item>
          <Descriptions.Item label="Team">
            {person.team || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Manager">
            {person.manager || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Notes" span={2}>
            {person.notes || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Created">
            {formatDate(person.created_at)}
          </Descriptions.Item>
          <Descriptions.Item label="Last Updated">
            {formatDate(person.updated_at)}
          </Descriptions.Item>
        </Descriptions>
      </Card>
    </div>
  );
};
