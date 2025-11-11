/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import {
  Layout,
  Typography,
  Card,
  Menu,
  theme,
} from 'antd';
import { invoke } from '@tauri-apps/api/core';
import {
  ProjectOutlined,
  UserOutlined,
  UsergroupAddOutlined,
  ClockCircleOutlined,
  TeamOutlined,
  FileTextOutlined,
  InfoCircleOutlined,
} from '@ant-design/icons';
import { ProjectList } from './components/ProjectList';
import { ProjectDetail } from './components/ProjectDetail';
import { ProjectForm } from './components/ProjectForm';
import { PersonList } from './components/PersonList';
import { PersonDetail } from './components/PersonDetail';
import { PersonForm } from './components/PersonForm';
import { TeamList } from './components/TeamList';
import { TeamDetail } from './components/TeamDetail';
import { TeamForm } from './components/TeamForm';
import { DeadlinesList } from './components/DeadlinesList';
import { Resources } from './components/Resources';
import { About } from './components/About';
import type { Project, Person, Team, Milestone } from './types';

const { Header, Content, Sider } = Layout;
const { Title } = Typography;

type ViewMode = 'list' | 'detail' | 'create' | 'edit';

function App() {
  const [selectedMenu, setSelectedMenu] = useState('1');
  const [viewMode, setViewMode] = useState<ViewMode>('list');
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);
  const [selectedPerson, setSelectedPerson] = useState<Person | null>(null);
  const [selectedTeam, setSelectedTeam] = useState<Team | null>(null);
  const [mcpPort, setMcpPort] = useState<number | null>(null);

  const {
    token: { colorBgContainer },
  } = theme.useToken();

  // Fetch MCP server port on mount
  useEffect(() => {
    const fetchMcpPort = async () => {
      try {
        const port = await invoke<number>('get_mcp_port');
        setMcpPort(port);
      } catch (error) {
        console.error('Failed to fetch MCP port:', error);
      }
    };
    fetchMcpPort();
  }, []);

  const handleViewProject = (project: Project) => {
    setSelectedProject(project);
    setViewMode('detail');
  };

  const handleEditProject = (project: Project) => {
    setSelectedProject(project);
    setViewMode('edit');
  };

  const handleCreateProject = () => {
    setSelectedProject(null);
    setViewMode('create');
  };

  const handleSaveProject = () => {
    setViewMode('list');
    setSelectedProject(null);
  };

  const handleCancelProject = () => {
    setViewMode('list');
    setSelectedProject(null);
  };

  const handleBackToList = () => {
    setViewMode('list');
    setSelectedProject(null);
  };

  const handleEditFromDetail = () => {
    setViewMode('edit');
  };

  const handleViewPerson = (person: Person) => {
    setSelectedPerson(person);
    setViewMode('detail');
  };

  const handleEditPerson = (person: Person) => {
    setSelectedPerson(person);
    setViewMode('edit');
  };

  const handleCreatePerson = () => {
    setSelectedPerson(null);
    setViewMode('create');
  };

  const handleSavePerson = () => {
    setViewMode('list');
    setSelectedPerson(null);
  };

  const handleCancelPerson = () => {
    setViewMode('list');
    setSelectedPerson(null);
  };

  const handleBackToPersonList = () => {
    setViewMode('list');
    setSelectedPerson(null);
  };

  const handleViewTeam = (team: Team) => {
    setSelectedTeam(team);
    setViewMode('detail');
  };

  const handleEditTeam = (team: Team) => {
    setSelectedTeam(team);
    setViewMode('edit');
  };

  const handleCreateTeam = () => {
    setSelectedTeam(null);
    setViewMode('create');
  };

  const handleSaveTeam = () => {
    setViewMode('list');
    setSelectedTeam(null);
  };

  const handleCancelTeam = () => {
    setViewMode('list');
    setSelectedTeam(null);
  };

  const handleBackToTeamList = () => {
    setViewMode('list');
    setSelectedTeam(null);
  };

  const handleViewProjectFromDeadlines = (project: Project) => {
    setSelectedMenu('1');
    setSelectedProject(project);
    setViewMode('detail');
  };

  const handleViewMilestoneFromDeadlines = async (milestone: Milestone) => {
    // Find the project for this milestone and navigate to it
    const { ProjectService } = await import('./services/projectService');
    try {
      const project = await ProjectService.getProject(milestone.project_id);
      if (project) {
        setSelectedMenu('1');
        setSelectedProject(project);
        setViewMode('detail');
      }
    } catch (error) {
      console.error('Failed to load project for milestone:', error);
    }
  };

  const menuItems = [
    { key: '1', icon: <ProjectOutlined />, label: 'Projects' },
    { key: '2', icon: <UserOutlined />, label: 'People' },
    { key: '3', icon: <UsergroupAddOutlined />, label: 'Teams' },
    { key: '4', icon: <ClockCircleOutlined />, label: 'Deadlines' },
    { key: '5', icon: <TeamOutlined />, label: 'Resources' },
    { key: '6', icon: <FileTextOutlined />, label: 'Reports' },
    { key: '7', icon: <InfoCircleOutlined />, label: 'About' },
  ];

  const renderContent = () => {
    // Projects section
    if (selectedMenu === '1') {
      switch (viewMode) {
        case 'detail':
          return selectedProject ? (
            <ProjectDetail
              projectId={selectedProject.id}
              onEdit={handleEditFromDetail}
              onBack={handleBackToList}
            />
          ) : null;

        case 'create':
          return (
            <ProjectForm
              onSave={handleSaveProject}
              onCancel={handleCancelProject}
            />
          );

        case 'edit':
          return selectedProject ? (
            <ProjectForm
              project={selectedProject}
              onSave={handleSaveProject}
              onCancel={handleCancelProject}
            />
          ) : null;

        case 'list':
        default:
          return (
            <ProjectList
              onViewProject={handleViewProject}
              onEditProject={handleEditProject}
              onCreateProject={handleCreateProject}
            />
          );
      }
    }

    // People section
    if (selectedMenu === '2') {
      switch (viewMode) {
        case 'detail':
          return selectedPerson ? (
            <PersonDetail
              person={selectedPerson}
              onEdit={handleEditFromDetail}
              onBack={handleBackToPersonList}
            />
          ) : null;

        case 'create':
          return (
            <PersonForm
              onSave={handleSavePerson}
              onCancel={handleCancelPerson}
            />
          );

        case 'edit':
          return selectedPerson ? (
            <PersonForm
              person={selectedPerson}
              onSave={handleSavePerson}
              onCancel={handleCancelPerson}
            />
          ) : null;

        case 'list':
        default:
          return (
            <PersonList
              onViewPerson={handleViewPerson}
              onEditPerson={handleEditPerson}
              onCreatePerson={handleCreatePerson}
            />
          );
      }
    }

    // Teams section
    if (selectedMenu === '3') {
      switch (viewMode) {
        case 'detail':
          return selectedTeam ? (
            <TeamDetail
              team={selectedTeam}
              onEdit={handleEditFromDetail}
              onBack={handleBackToTeamList}
              onViewProject={handleViewProject}
            />
          ) : null;

        case 'create':
          return (
            <TeamForm
              onSave={handleSaveTeam}
              onCancel={handleCancelTeam}
            />
          );

        case 'edit':
          return selectedTeam ? (
            <TeamForm
              team={selectedTeam}
              onSave={handleSaveTeam}
              onCancel={handleCancelTeam}
            />
          ) : null;

        case 'list':
        default:
          return (
            <TeamList
              onViewTeam={handleViewTeam}
              onEditTeam={handleEditTeam}
              onCreateTeam={handleCreateTeam}
            />
          );
      }
    }

    // Deadlines section
    if (selectedMenu === '4') {
      return (
        <DeadlinesList
          onViewProject={handleViewProjectFromDeadlines}
          onViewMilestone={handleViewMilestoneFromDeadlines}
        />
      );
    }

    // Resources section
    if (selectedMenu === '5') {
      return <Resources />;
    }

    // About section
    if (selectedMenu === '7') {
      return <About />;
    }

    // Other sections - coming soon
    return (
      <Card title={<Title level={4}>Coming Soon</Title>}>
        <p>This feature is under development.</p>
      </Card>
    );
  };

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Header style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '0 24px' }}>
        <Title level={3} style={{ color: 'white', margin: 0 }}>
          Project Tracker
        </Title>
        {mcpPort && (
          <Typography.Text style={{ color: 'white' }}>
            MCP: http://127.0.0.1:{mcpPort}/sse
          </Typography.Text>
        )}
      </Header>
      <Layout>
        <Sider width={200} style={{ background: colorBgContainer }}>
          <Menu
            mode="inline"
            selectedKeys={[selectedMenu]}
            onClick={(e) => {
              setSelectedMenu(e.key);
              setViewMode('list');
              setSelectedProject(null);
              setSelectedPerson(null);
              setSelectedTeam(null);
            }}
            style={{ height: '100%', borderRight: 0 }}
            items={menuItems}
          />
        </Sider>
        <Layout style={{ padding: '24px' }}>
          <Content>
            {renderContent()}
          </Content>
        </Layout>
      </Layout>
    </Layout>
  );
}

export default App;
