/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Form, Input, Button, message, Space } from 'antd';
import { SaveOutlined, CloseOutlined } from '@ant-design/icons';

const { TextArea } = Input;

interface Note {
  id: string;
  title: string;
  body: string;
  created_at: string;
  updated_at: string;
}

interface NoteFormProps {
  note?: Note;
  onSave: (title: string, body: string, noteId?: string) => Promise<void>;
  onCancel: () => void;
}

export const NoteForm: React.FC<NoteFormProps> = ({ note, onSave, onCancel }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const isEditMode = !!note;

  useEffect(() => {
    if (note) {
      form.setFieldsValue({
        title: note.title,
        body: note.body,
      });
    } else {
      form.resetFields();
    }
  }, [note, form]);

  const handleSubmit = async (values: any) => {
    setLoading(true);
    try {
      await onSave(values.title, values.body, note?.id);
      if (!isEditMode) {
        form.resetFields();
      }
    } catch (error) {
      message.error('Failed to save note: ' + error);
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
        title: '',
        body: '',
      }}
    >
      <Form.Item
        name="title"
        label="Title"
        rules={[{ required: true, message: 'Please enter a title' }]}
      >
        <Input placeholder="Enter note title" />
      </Form.Item>

      <Form.Item
        name="body"
        label="Note"
        rules={[{ required: true, message: 'Please enter note content' }]}
      >
        <TextArea rows={6} placeholder="Enter note content" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            htmlType="submit"
            icon={<SaveOutlined />}
            loading={loading}
          >
            {isEditMode ? 'Update Note' : 'Add Note'}
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
