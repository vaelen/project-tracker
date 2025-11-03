/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { Table, Button, Space, Modal, message } from 'antd';
import { EyeOutlined, DeleteOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import type { Note } from '../types';

interface NoteListProps {
  notes: Note[];
  onView: (note: Note) => void;
  onDelete: (id: string) => Promise<void>;
  loading?: boolean;
}

export const NoteList: React.FC<NoteListProps> = ({ notes, onView, onDelete, loading = false }) => {
  const handleDelete = (note: Note) => {
    Modal.confirm({
      title: 'Delete Note',
      content: `Are you sure you want to delete "${note.title}"?`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await onDelete(note.id);
          message.success('Note deleted successfully');
        } catch (error) {
          message.error('Failed to delete note: ' + error);
        }
      },
    });
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString();
  };

  const columns: ColumnsType<Note> = [
    {
      title: 'Title',
      dataIndex: 'title',
      key: 'title',
      ellipsis: true,
    },
    {
      title: 'Created',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 120,
      render: (date: string) => formatDate(date),
    },
    {
      title: 'Updated',
      dataIndex: 'updated_at',
      key: 'updated_at',
      width: 120,
      render: (date: string) => formatDate(date),
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 150,
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EyeOutlined />}
            onClick={() => onView(record)}
          >
            View
          </Button>
          <Button
            type="link"
            danger
            size="small"
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
    <Table
      columns={columns}
      dataSource={notes}
      rowKey="id"
      loading={loading}
      locale={{ emptyText: 'No notes yet' }}
      pagination={{ pageSize: 10 }}
    />
  );
};
