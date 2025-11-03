/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Table, message, Typography, Tag } from 'antd';
import { ProjectOutlined, FlagOutlined, LinkOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { ProjectService } from '../services/projectService';
import type { Project, Milestone } from '../types';

const { Link } = Typography;

interface DeadlineItem {
  id: string;
  type: 'project' | 'milestone';
  name: string;
  dueDate: string | null;
  ticket: string | null;
  projectId?: string;
}

interface DeadlinesListProps {
  onViewProject: (project: Project) => void;
  onViewMilestone: (milestone: Milestone) => void;
}

export const DeadlinesList: React.FC<DeadlinesListProps> = ({
  onViewProject,
  onViewMilestone,
}) => {
  const [deadlines, setDeadlines] = useState<DeadlineItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [jiraBaseUrl, setJiraBaseUrl] = useState<string>('');
  const [projects, setProjects] = useState<Project[]>([]);
  const [milestones, setMilestones] = useState<Milestone[]>([]);

  useEffect(() => {
    loadDeadlines();
    loadJiraUrl();
  }, []);

  const loadJiraUrl = async () => {
    try {
      const url = await ProjectService.getJiraUrl();
      setJiraBaseUrl(url);
    } catch (error) {
      console.error('Failed to load Jira URL:', error);
    }
  };

  const loadDeadlines = async () => {
    setLoading(true);
    try {
      // Load all projects
      const projectsData = await ProjectService.listProjects();
      setProjects(projectsData);

      // Load milestones for all projects
      const allMilestones: Milestone[] = [];
      for (const project of projectsData) {
        const projectMilestones = await ProjectService.getProjectMilestones(project.id);
        allMilestones.push(...projectMilestones);
      }
      setMilestones(allMilestones);

      // Combine projects and milestones into deadline items
      const deadlineItems: DeadlineItem[] = [];

      // Add projects with due dates
      for (const project of projectsData) {
        if (project.due_date) {
          deadlineItems.push({
            id: project.id,
            type: 'project',
            name: project.name,
            dueDate: project.due_date,
            ticket: project.jira_initiative || null,
          });
        }
      }

      // Add milestones with due dates
      for (const milestone of allMilestones) {
        if (milestone.due_date) {
          deadlineItems.push({
            id: milestone.id,
            type: 'milestone',
            name: milestone.name,
            dueDate: milestone.due_date,
            ticket: milestone.jira_epic || null,
            projectId: milestone.project_id,
          });
        }
      }

      // Sort by due date (earliest first)
      deadlineItems.sort((a, b) => {
        if (!a.dueDate) return 1;
        if (!b.dueDate) return -1;
        return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
      });

      setDeadlines(deadlineItems);
    } catch (error) {
      message.error('Failed to load deadlines: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateString: string | null) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString();
  };

  const handleViewItem = (record: DeadlineItem) => {
    if (record.type === 'project') {
      const project = projects.find(p => p.id === record.id);
      if (project) {
        onViewProject(project);
      }
    } else {
      const milestone = milestones.find(m => m.id === record.id);
      if (milestone) {
        onViewMilestone(milestone);
      }
    }
  };

  const columns: ColumnsType<DeadlineItem> = [
    {
      title: 'Due Date',
      dataIndex: 'dueDate',
      key: 'dueDate',
      width: 120,
      render: formatDate,
      sorter: (a, b) => {
        if (!a.dueDate) return 1;
        if (!b.dueDate) return -1;
        return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
      },
      defaultSortOrder: 'ascend',
    },
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
      width: 100,
      render: (type: 'project' | 'milestone') => {
        if (type === 'project') {
          return (
            <Tag icon={<ProjectOutlined />} color="blue">
              Project
            </Tag>
          );
        }
        return (
          <Tag icon={<FlagOutlined />} color="green">
            Milestone
          </Tag>
        );
      },
      filters: [
        { text: 'Project', value: 'project' },
        { text: 'Milestone', value: 'milestone' },
      ],
      onFilter: (value, record) => record.type === value,
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      ellipsis: true,
      render: (name: string, record: DeadlineItem) => (
        <Link onClick={() => handleViewItem(record)} style={{ cursor: 'pointer' }}>
          {name}
        </Link>
      ),
    },
    {
      title: 'Ticket',
      dataIndex: 'ticket',
      key: 'ticket',
      width: 150,
      render: (ticket: string | null) => ticket ? (
        <Link href={`${jiraBaseUrl}${ticket}`} target="_blank">
          <LinkOutlined /> {ticket}
        </Link>
      ) : '-',
    },
  ];

  return (
    <div>
      <Table
        columns={columns}
        dataSource={deadlines}
        rowKey="id"
        loading={loading}
        pagination={{
          showSizeChanger: true,
          showTotal: (total) => `Total ${total} deadlines`,
        }}
      />
    </div>
  );
};
