/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect, useRef } from 'react';
import { Card, Input, Button, List, message, Modal, Form, Space, Typography } from 'antd';
import { SendOutlined, UserOutlined, RobotOutlined, KeyOutlined } from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';
import Markdown from 'react-markdown';

const { TextArea } = Input;
const { Text, Title, Paragraph } = Typography;

interface Message {
  role: 'user' | 'assistant';
  content: string;
}

interface AuthStatus {
  authenticated: boolean;
  organization_id?: string | null;
}

export const ChatInterface: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [loading, setLoading] = useState(false);
  const [authenticated, setAuthenticated] = useState(false);
  const [showAuthModal, setShowAuthModal] = useState(false);
  const [authForm] = Form.useForm();
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    checkAuthStatus();
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const checkAuthStatus = async () => {
    try {
      const status = await invoke<AuthStatus>('check_auth_status');
      setAuthenticated(status.authenticated);

      if (!status.authenticated) {
        setShowAuthModal(true);
      }
    } catch (error) {
      console.error('Failed to check auth status:', error);
      setShowAuthModal(true);
    }
  };

  const handleOpenAuthUrl = async () => {
    try {
      await invoke('open_auth_url');
      message.info('Opening authentication page in your browser...');
    } catch (error) {
      message.error('Failed to open auth URL: ' + error);
    }
  };

  const handleSaveCredentials = async (values: any) => {
    try {
      await invoke('save_credentials', {
        sessionKey: values.session_key,
        organizationId: values.organization_id || null,
      });

      message.success('Credentials saved successfully!');
      setAuthenticated(true);
      setShowAuthModal(false);
      authForm.resetFields();
    } catch (error) {
      message.error('Failed to save credentials: ' + error);
    }
  };

  const handleSendMessage = async () => {
    if (!inputValue.trim()) return;

    if (!authenticated) {
      message.warning('Please authenticate first');
      setShowAuthModal(true);
      return;
    }

    const userMessage: Message = {
      role: 'user',
      content: inputValue,
    };

    setMessages(prev => [...prev, userMessage]);
    setInputValue('');
    setLoading(true);

    try {
      // Prepare messages for API (include conversation history)
      const apiMessages = [...messages, userMessage];

      const response = await invoke<string>('send_chat_message', {
        messages: apiMessages,
      });

      const assistantMessage: Message = {
        role: 'assistant',
        content: response,
      };

      setMessages(prev => [...prev, assistantMessage]);
    } catch (error) {
      message.error('Failed to send message: ' + error);

      // If authentication error, show auth modal
      if (String(error).includes('authenticated')) {
        setAuthenticated(false);
        setShowAuthModal(true);
      }
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div style={{ height: 'calc(100vh - 200px)', display: 'flex', flexDirection: 'column' }}>
      <Card
        title={
          <Space>
            <Title level={4} style={{ margin: 0 }}>Chat with Claude</Title>
            {!authenticated && (
              <Button
                type="primary"
                icon={<KeyOutlined />}
                size="small"
                onClick={() => setShowAuthModal(true)}
              >
                Authenticate
              </Button>
            )}
          </Space>
        }
        style={{ flex: 1, display: 'flex', flexDirection: 'column' }}
        bodyStyle={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}
      >
        <div style={{ flex: 1, overflowY: 'auto', marginBottom: 16, padding: '8px 0' }}>
          {messages.length === 0 ? (
            <div style={{ textAlign: 'center', padding: '40px', color: '#888' }}>
              <RobotOutlined style={{ fontSize: 48, marginBottom: 16 }} />
              <Paragraph>Start a conversation with Claude AI</Paragraph>
              <Paragraph type="secondary">
                Ask me anything about your projects, get insights, or just chat!
              </Paragraph>
            </div>
          ) : (
            <List
              dataSource={messages}
              renderItem={(msg, index) => (
                <List.Item
                  key={index}
                  style={{
                    justifyContent: msg.role === 'user' ? 'flex-end' : 'flex-start',
                    border: 'none',
                    padding: '8px 0',
                  }}
                >
                  <div
                    style={{
                      maxWidth: '80%',
                      padding: '12px 16px',
                      borderRadius: '8px',
                      backgroundColor: msg.role === 'user' ? '#1890ff' : '#f0f0f0',
                      color: msg.role === 'user' ? 'white' : 'black',
                    }}
                  >
                    <Space direction="vertical" size="small" style={{ width: '100%' }}>
                      <Space>
                        {msg.role === 'user' ? <UserOutlined /> : <RobotOutlined />}
                        <Text strong style={{ color: msg.role === 'user' ? 'white' : 'inherit' }}>
                          {msg.role === 'user' ? 'You' : 'Claude'}
                        </Text>
                      </Space>
                      {msg.role === 'assistant' ? (
                        <Markdown>{msg.content}</Markdown>
                      ) : (
                        <Text style={{ color: 'white', whiteSpace: 'pre-wrap' }}>
                          {msg.content}
                        </Text>
                      )}
                    </Space>
                  </div>
                </List.Item>
              )}
            />
          )}
          <div ref={messagesEndRef} />
        </div>

        <Space.Compact style={{ width: '100%' }}>
          <TextArea
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Type your message here... (Press Enter to send, Shift+Enter for new line)"
            autoSize={{ minRows: 1, maxRows: 4 }}
            disabled={loading || !authenticated}
            style={{ flex: 1 }}
          />
          <Button
            type="primary"
            icon={<SendOutlined />}
            onClick={handleSendMessage}
            loading={loading}
            disabled={!authenticated}
          >
            Send
          </Button>
        </Space.Compact>
      </Card>

      <Modal
        title="Authenticate with Claude"
        open={showAuthModal}
        onCancel={() => authenticated && setShowAuthModal(false)}
        footer={null}
        closable={authenticated}
        maskClosable={authenticated}
      >
        <Paragraph>
          To use Claude chat, you need to authenticate with your Anthropic API key.
        </Paragraph>

        <Space direction="vertical" size="large" style={{ width: '100%' }}>
          <div>
            <Paragraph strong>Step 1: Get your API Key</Paragraph>
            <Button type="primary" onClick={handleOpenAuthUrl} block>
              Open Claude Console
            </Button>
            <Paragraph type="secondary" style={{ marginTop: 8, fontSize: '12px' }}>
              Sign in to your Anthropic account and navigate to API Keys in your account settings.
              Create a new API key if you don't have one.
            </Paragraph>
          </div>

          <div>
            <Paragraph strong>Step 2: Enter your credentials</Paragraph>
            <Form form={authForm} layout="vertical" onFinish={handleSaveCredentials}>
              <Form.Item
                label="API Key (Session Key)"
                name="session_key"
                rules={[{ required: true, message: 'Please enter your API key' }]}
              >
                <TextArea
                  rows={3}
                  placeholder="sk-ant-..."
                />
              </Form.Item>

              <Form.Item
                label="Organization ID (Optional)"
                name="organization_id"
              >
                <Input placeholder="org-..." />
              </Form.Item>

              <Form.Item>
                <Button type="primary" htmlType="submit" block>
                  Save Credentials
                </Button>
              </Form.Item>
            </Form>
          </div>
        </Space>
      </Modal>
    </div>
  );
};
