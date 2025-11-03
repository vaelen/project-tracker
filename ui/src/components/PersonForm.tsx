/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Form, Input, Button, Card, message, Select, Space } from 'antd';
import { SaveOutlined, CloseOutlined } from '@ant-design/icons';
import { PersonService } from '../services/personService';
import type { Person } from '../types';

const { TextArea } = Input;

interface PersonFormProps {
  person?: Person;
  onSave: (person: Person) => void;
  onCancel: () => void;
  isModal?: boolean;
}

export const PersonForm: React.FC<PersonFormProps> = ({ person, onSave, onCancel, isModal = false }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [people, setPeople] = useState<Person[]>([]);
  const [defaultDomain, setDefaultDomain] = useState('');

  const isEditing = !!person && !!person.email;

  useEffect(() => {
    loadPeople();
    loadDefaultDomain();

    if (person) {
      form.setFieldsValue(person);
    }
  }, [person]);

  const loadPeople = async () => {
    try {
      const data = await PersonService.listPeople();
      setPeople(data);
    } catch (error) {
      message.error('Failed to load people: ' + error);
    }
  };

  const loadDefaultDomain = async () => {
    try {
      const domain = await PersonService.getDefaultEmailDomain();
      setDefaultDomain(domain);
    } catch (error) {
      console.error('Failed to load default domain:', error);
    }
  };

  const handleSubmit = async (values: any) => {
    setLoading(true);
    try {
      const personData: Person = {
        email: values.email,
        name: values.name,
        team: values.team || undefined,
        manager: values.manager || undefined,
        notes: values.notes || undefined,
        created_at: person?.created_at || new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      if (isEditing) {
        await PersonService.updatePerson(personData);
        message.success('Person updated successfully');
      } else {
        await PersonService.createPerson(personData);
        message.success('Person created successfully');
      }

      onSave(personData);
    } catch (error) {
      message.error(`Failed to ${isEditing ? 'update' : 'create'} person: ` + error);
    } finally {
      setLoading(false);
    }
  };

  const managerOptions = people
    .filter(p => !person || p.email !== person.email) // Don't let someone be their own manager
    .map(p => ({
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
        email: '',
        team: '',
        manager: undefined,
        notes: '',
      }}
    >
      <Form.Item
        name="name"
        label="Name"
        rules={[{ required: true, message: 'Please enter a name' }]}
      >
        <Input placeholder="Enter person's name" />
      </Form.Item>

      <Form.Item
        name="email"
        label="Email"
        rules={[
          { required: true, message: 'Please enter an email' },
          { type: 'email', message: 'Please enter a valid email' },
        ]}
        extra={defaultDomain && !isEditing ? `Suggested domain: @${defaultDomain}` : undefined}
      >
        <Input placeholder={`e.g., user@${defaultDomain}`} disabled={isEditing} />
      </Form.Item>

      <Form.Item
        name="team"
        label="Team"
      >
        <Input placeholder="Enter team name" />
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

      <Form.Item
        name="notes"
        label="Notes"
      >
        <TextArea rows={4} placeholder="Enter any additional notes" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            htmlType="submit"
            icon={<SaveOutlined />}
            loading={loading}
          >
            {isEditing ? 'Update' : 'Create'} Person
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
    <Card title={isEditing ? 'Edit Person' : 'New Person'}>
      {formContent}
    </Card>
  );
};
