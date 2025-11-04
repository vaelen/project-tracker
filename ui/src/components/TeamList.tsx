/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Table, Button, Space, message, Modal } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, EyeOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { TeamService } from '../services/teamService';
import type { Team } from '../types';

interface TeamListProps {
  onEditTeam: (team: Team) => void;
  onCreateTeam: () => void;
  onViewTeam: (team: Team) => void;
}

export const TeamList: React.FC<TeamListProps> = ({
  onEditTeam,
  onCreateTeam,
  onViewTeam,
}) => {
  const [teams, setTeams] = useState<Team[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadTeams();
  }, []);

  const loadTeams = async () => {
    setLoading(true);
    try {
      const data = await TeamService.listTeams();
      setTeams(data);
    } catch (error) {
      message.error('Failed to load teams: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (team: Team) => {
    Modal.confirm({
      title: 'Delete Team',
      content: `Are you sure you want to delete "${team.name}"? This may affect associated projects and milestones.`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await TeamService.deleteTeam(team.name);
          message.success('Team deleted successfully');
          loadTeams();
        } catch (error) {
          message.error('Failed to delete team: ' + error);
        }
      },
    });
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString();
  };

  const columns: ColumnsType<Team> = [
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      width: 200,
      sorter: (a, b) => a.name.localeCompare(b.name),
      fixed: 'left',
    },
    {
      title: 'Manager',
      dataIndex: 'manager',
      key: 'manager',
      width: 200,
      render: (text) => text || '-',
    },
    {
      title: 'Description',
      dataIndex: 'description',
      key: 'description',
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
            onClick={() => onViewTeam(record)}
          >
            View
          </Button>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => onEditTeam(record)}
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
          onClick={onCreateTeam}
        >
          New Team
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={teams}
        rowKey="name"
        loading={loading}
        scroll={{ x: 1000 }}
        pagination={{
          showSizeChanger: true,
          showTotal: (total) => `Total ${total} teams`,
        }}
      />
    </div>
  );
};
