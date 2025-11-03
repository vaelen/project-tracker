/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Table, Button, Space, message, Modal } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, EyeOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { PersonService } from '../services/personService';
import type { Person } from '../types';

interface PersonListProps {
  onEditPerson: (person: Person) => void;
  onCreatePerson: () => void;
  onViewPerson: (person: Person) => void;
}

export const PersonList: React.FC<PersonListProps> = ({
  onEditPerson,
  onCreatePerson,
  onViewPerson,
}) => {
  const [people, setPeople] = useState<Person[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadPeople();
  }, []);

  const loadPeople = async () => {
    setLoading(true);
    try {
      const data = await PersonService.listPeople();
      setPeople(data);
    } catch (error) {
      message.error('Failed to load people: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (person: Person) => {
    Modal.confirm({
      title: 'Delete Person',
      content: `Are you sure you want to delete "${person.name}"? This may affect associated projects.`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await PersonService.deletePerson(person.email);
          message.success('Person deleted successfully');
          loadPeople();
        } catch (error) {
          message.error('Failed to delete person: ' + error);
        }
      },
    });
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString();
  };

  const columns: ColumnsType<Person> = [
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      width: 200,
      sorter: (a, b) => a.name.localeCompare(b.name),
      fixed: 'left',
    },
    {
      title: 'Email',
      dataIndex: 'email',
      key: 'email',
      width: 250,
      sorter: (a, b) => a.email.localeCompare(b.email),
    },
    {
      title: 'Team',
      dataIndex: 'team',
      key: 'team',
      width: 150,
      render: (text) => text || '-',
    },
    {
      title: 'Manager',
      dataIndex: 'manager',
      key: 'manager',
      width: 200,
      render: (text) => text || '-',
    },
    {
      title: 'Notes',
      dataIndex: 'notes',
      key: 'notes',
      ellipsis: true,
      render: (text) => text || '-',
    },
    {
      title: 'Created',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 120,
      render: formatDate,
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 230,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EyeOutlined />}
            onClick={() => onViewPerson(record)}
          >
            View
          </Button>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => onEditPerson(record)}
          >
            Edit
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDelete(record)}
          >
            Delete
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={onCreatePerson}
        >
          New Person
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={people}
        rowKey="email"
        loading={loading}
        scroll={{ x: 1200 }}
        pagination={{
          showSizeChanger: true,
          showTotal: (total) => `Total ${total} people`,
        }}
      />
    </div>
  );
};
