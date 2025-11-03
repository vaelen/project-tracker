/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { Modal, Button, Space, Typography } from 'antd';
import { EditOutlined, DeleteOutlined } from '@ant-design/icons';
import Markdown from 'react-markdown';

const { Text } = Typography;

interface Note {
  id: string;
  title: string;
  body: string;
  created_at: string;
  updated_at: string;
}

interface NoteViewModalProps {
  note: Note | null;
  open: boolean;
  onClose: () => void;
  onEdit: (note: Note) => void;
  onDelete: (id: string) => void;
}

export const NoteViewModal: React.FC<NoteViewModalProps> = ({
  note,
  open,
  onClose,
  onEdit,
  onDelete,
}) => {
  if (!note) return null;

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString();
  };

  const handleEdit = () => {
    onEdit(note);
  };

  const handleDelete = () => {
    onDelete(note.id);
  };

  return (
    <Modal
      title={note.title}
      open={open}
      onCancel={onClose}
      width={800}
      footer={[
        <Button key="edit" type="primary" icon={<EditOutlined />} onClick={handleEdit}>
          Edit
        </Button>,
        <Button key="delete" danger icon={<DeleteOutlined />} onClick={handleDelete}>
          Delete
        </Button>,
        <Button key="close" onClick={onClose}>
          Close
        </Button>,
      ]}
    >
      <Space direction="vertical" size="middle" style={{ width: '100%' }}>
        <div style={{ marginBottom: '16px' }}>
          <Text type="secondary" style={{ fontSize: '12px' }}>
            Created: {formatDate(note.created_at)}
          </Text>
          <br />
          <Text type="secondary" style={{ fontSize: '12px' }}>
            Updated: {formatDate(note.updated_at)}
          </Text>
        </div>

        <div style={{
          padding: '16px',
          backgroundColor: '#f5f5f5',
          borderRadius: '4px',
          maxHeight: '500px',
          overflowY: 'auto'
        }}>
          <Markdown>{note.body}</Markdown>
        </div>
      </Space>
    </Modal>
  );
};
