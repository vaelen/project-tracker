/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  Layout,
  Typography,
  Card,
  Input,
  Button,
  List,
  Space,
  Alert,
  Menu,
  theme,
} from 'antd';
import {
  ProjectOutlined,
  TeamOutlined,
  ClockCircleOutlined,
  RocketOutlined,
  UserOutlined,
  MessageOutlined,
  FileTextOutlined,
  PlusOutlined,
  ReloadOutlined,
} from '@ant-design/icons';

const { Header, Content, Sider } = Layout;
const { Title, Paragraph } = Typography;

function App() {
  const [projects, setProjects] = useState<string[]>([]);
  const [newProjectName, setNewProjectName] = useState('');
  const [message, setMessage] = useState('');
  const [selectedMenu, setSelectedMenu] = useState('1');

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  const loadProjects = async () => {
    try {
      const result = await invoke<string[]>('list_projects');
      setProjects(result);
    } catch (error) {
      console.error('Failed to load projects:', error);
    }
  };

  const createProject = async () => {
    if (!newProjectName.trim()) return;

    try {
      const result = await invoke<string>('create_project', { name: newProjectName });
      setMessage(result);
      setNewProjectName('');
      loadProjects();
    } catch (error) {
      console.error('Failed to create project:', error);
    }
  };

  const menuItems = [
    { key: '1', icon: <ProjectOutlined />, label: 'Projects' },
    { key: '2', icon: <TeamOutlined />, label: 'Employees' },
    { key: '3', icon: <ClockCircleOutlined />, label: 'Deadlines' },
    { key: '4', icon: <RocketOutlined />, label: 'Initiatives' },
    { key: '5', icon: <UserOutlined />, label: 'Stakeholders' },
    { key: '6', icon: <MessageOutlined />, label: 'Claude AI' },
    { key: '7', icon: <FileTextOutlined />, label: 'Reports' },
  ];

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Header style={{ display: 'flex', alignItems: 'center', padding: '0 24px' }}>
        <Title level={3} style={{ color: 'white', margin: 0 }}>
          Claude Tracker
        </Title>
      </Header>
      <Layout>
        <Sider width={200} style={{ background: colorBgContainer }}>
          <Menu
            mode="inline"
            selectedKeys={[selectedMenu]}
            onClick={(e) => setSelectedMenu(e.key)}
            style={{ height: '100%', borderRight: 0 }}
            items={menuItems}
          />
        </Sider>
        <Layout style={{ padding: '24px' }}>
          <Content>
            <Card
              title={<Title level={4}>Projects</Title>}
              style={{ marginBottom: 24 }}
            >
              <Paragraph>
                Intelligent project and resource management for engineering managers
              </Paragraph>

              <Space.Compact style={{ width: '100%', marginBottom: 16 }}>
                <Input
                  placeholder="New project name"
                  value={newProjectName}
                  onChange={(e) => setNewProjectName(e.target.value)}
                  onPressEnter={createProject}
                />
                <Button type="primary" icon={<PlusOutlined />} onClick={createProject}>
                  Add
                </Button>
                <Button icon={<ReloadOutlined />} onClick={loadProjects}>
                  Refresh
                </Button>
              </Space.Compact>

              {message && (
                <Alert
                  message={message}
                  type="success"
                  showIcon
                  closable
                  onClose={() => setMessage('')}
                  style={{ marginBottom: 16 }}
                />
              )}

              <List
                bordered
                dataSource={projects.length > 0 ? projects : ['No projects yet. Add one above!']}
                renderItem={(item) => (
                  <List.Item>
                    <ProjectOutlined style={{ marginRight: 8 }} />
                    {item}
                  </List.Item>
                )}
              />
            </Card>

            <Card title={<Title level={4}>Coming Soon</Title>}>
              <List
                dataSource={[
                  'Employee Management',
                  'Deadline Tracking',
                  'Initiative Planning',
                  'Stakeholder Management',
                  'Claude AI Chat Integration',
                  'Status Report Generation',
                ]}
                renderItem={(item) => <List.Item>{item}</List.Item>}
              />
            </Card>
          </Content>
        </Layout>
      </Layout>
    </Layout>
  );
}

export default App;
