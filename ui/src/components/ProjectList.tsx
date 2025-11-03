/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Table, Button, Space, message, Modal, Tag, Typography } from 'antd';
import { PlusOutlined, EyeOutlined, EditOutlined, DeleteOutlined, LinkOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { ProjectService } from '../services/projectService';
import type { Project } from '../types';

const { Link } = Typography;

interface ProjectListProps {
  onViewProject: (project: Project) => void;
  onEditProject: (project: Project) => void;
  onCreateProject: () => void;
}

export const ProjectList: React.FC<ProjectListProps> = ({
  onViewProject,
  onEditProject,
  onCreateProject,
}) => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(false);
  const [jiraBaseUrl, setJiraBaseUrl] = useState<string>('');

  useEffect(() => {
    loadProjects();
    loadJiraUrl();
  }, []);

  const loadProjects = async () => {
    setLoading(true);
    try {
      const data = await ProjectService.listProjects();
      setProjects(data);
    } catch (error) {
      message.error('Failed to load projects: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const loadJiraUrl = async () => {
    try {
      const url = await ProjectService.getJiraUrl();
      setJiraBaseUrl(url);
    } catch (error) {
      console.error('Failed to load Jira URL:', error);
    }
  };

  const handleDelete = async (project: Project) => {
    Modal.confirm({
      title: 'Delete Project',
      content: `Are you sure you want to delete "${project.name}"? This will also delete all associated milestones.`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await ProjectService.deleteProject(project.id);
          message.success('Project deleted successfully');
          loadProjects();
        } catch (error) {
          message.error('Failed to delete project: ' + error);
        }
      },
    });
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString();
  };

  const columns: ColumnsType<Project> = [
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      width: 200,
      sorter: (a, b) => a.name.localeCompare(b.name),
      fixed: 'left',
    },
    {
      title: 'Description',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
      width: 300,
      render: (text) => text || '-',
    },
    {
      title: 'Manager',
      dataIndex: 'manager',
      key: 'manager',
      width: 180,
      ellipsis: true,
      render: (email) => email || '-',
    },
    {
      title: 'Technical Lead',
      dataIndex: 'technical_lead',
      key: 'technical_lead',
      width: 180,
      ellipsis: true,
      render: (email) => email || '-',
    },
    {
      title: 'Due Date',
      dataIndex: 'due_date',
      key: 'due_date',
      width: 120,
      render: formatDate,
      sorter: (a, b) => {
        const dateA = a.due_date ? new Date(a.due_date).getTime() : 0;
        const dateB = b.due_date ? new Date(b.due_date).getTime() : 0;
        return dateA - dateB;
      },
    },
    {
      title: 'Jira Initiative',
      dataIndex: 'jira_initiative',
      key: 'jira_initiative',
      width: 150,
      render: (ticket) => ticket ? (
        <Link href={`${jiraBaseUrl}${ticket}`} target="_blank">
          <LinkOutlined /> {ticket}
        </Link>
      ) : '-',
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 240,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EyeOutlined />}
            onClick={() => onViewProject(record)}
          >
            View
          </Button>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => onEditProject(record)}
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
          onClick={onCreateProject}
        >
          New Project
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={projects}
        rowKey="id"
        loading={loading}
        scroll={{ x: 1400 }}
        pagination={{
          showSizeChanger: true,
          showTotal: (total) => `Total ${total} projects`,
        }}
      />
    </div>
  );
};
