/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { Card, Typography, Divider, Space } from 'antd';
import { GithubOutlined } from '@ant-design/icons';

const { Title, Paragraph, Text, Link } = Typography;

export function About() {
  const version = '0.1.0';
  const copyright = 'Copyright 2025 Andrew C. Young <andrew@vaelen.org>';
  const githubUrl = 'https://github.com/vaelen/project-tracker';
  const description = 'An intelligent project and resource management application for software engineering managers with AI integration via Model Context Protocol (MCP).';

  const licenseText = `MIT License

Copyright 2025 Andrew C. Young <andrew@vaelen.org>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.`;

  return (
    <Card>
      <Space direction="vertical" size="large" style={{ width: '100%' }}>
        <div style={{ textAlign: 'center' }}>
          <Title level={2}>Project Tracker</Title>
          <Text type="secondary">Version {version}</Text>
        </div>

        <Divider />

        <div>
          <Title level={4}>About</Title>
          <Paragraph>{description}</Paragraph>
        </div>

        <div>
          <Title level={4}>Copyright</Title>
          <Paragraph>{copyright}</Paragraph>
        </div>

        <div>
          <Title level={4}>Source Code</Title>
          <Paragraph>
            <Link href={githubUrl} target="_blank" rel="noopener noreferrer">
              <GithubOutlined /> {githubUrl}
            </Link>
          </Paragraph>
        </div>

        <Divider />

        <div>
          <Title level={4}>License</Title>
          <Paragraph>
            <pre style={{
              backgroundColor: '#f5f5f5',
              padding: '16px',
              borderRadius: '4px',
              fontSize: '12px',
              overflowX: 'auto'
            }}>
              {licenseText}
            </pre>
          </Paragraph>
        </div>
      </Space>
    </Card>
  );
}
