/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect } from 'react';
import { Card, Descriptions, Button, Space, Table, message, Typography, Spin, Modal } from 'antd';
import { EditOutlined, ArrowLeftOutlined, LinkOutlined, PlusOutlined, DeleteOutlined, EyeOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { ProjectService } from '../services/projectService';
import { MilestoneService } from '../services/milestoneService';
import { PersonService } from '../services/personService';
import { NoteService } from '../services/noteService';
import { MilestoneForm } from './MilestoneForm';
import { StakeholderForm } from './StakeholderForm';
import { ProjectResourceForm } from './ProjectResourceForm';
import { MilestoneResourceForm } from './MilestoneResourceForm';
import { NoteForm } from './NoteForm';
import { NoteList } from './NoteList';
import { NoteViewModal } from './NoteViewModal';
import type { Project, Milestone, ProjectStakeholder, ProjectResource, MilestoneResource, Person, Note, ProjectNote, MilestoneNote, StakeholderNote } from '../types';

const { Title, Link } = Typography;

interface ProjectDetailProps {
  projectId: string;
  onEdit: () => void;
  onBack: () => void;
}

export const ProjectDetail: React.FC<ProjectDetailProps> = ({ projectId, onEdit, onBack }) => {
  const [project, setProject] = useState<Project | null>(null);
  const [milestones, setMilestones] = useState<Milestone[]>([]);
  const [stakeholders, setStakeholders] = useState<ProjectStakeholder[]>([]);
  const [projectResources, setProjectResources] = useState<ProjectResource[]>([]);
  const [milestoneResources, setMilestoneResources] = useState<Map<string, MilestoneResource[]>>(new Map());
  const [people, setPeople] = useState<Person[]>([]);
  const [projectNotes, setProjectNotes] = useState<ProjectNote[]>([]);
  const [jiraBaseUrl, setJiraBaseUrl] = useState<string>('');
  const [loading, setLoading] = useState(false);

  // Milestone modal state
  const [showMilestoneModal, setShowMilestoneModal] = useState(false);
  const [selectedMilestone, setSelectedMilestone] = useState<Milestone | undefined>(undefined);

  // Stakeholder modal state
  const [showStakeholderModal, setShowStakeholderModal] = useState(false);
  const [selectedStakeholder, setSelectedStakeholder] = useState<ProjectStakeholder | undefined>(undefined);

  // Project resource modal state
  const [showProjectResourceModal, setShowProjectResourceModal] = useState(false);
  const [selectedProjectResource, setSelectedProjectResource] = useState<ProjectResource | undefined>(undefined);

  // Milestone resource modal state
  const [showMilestoneResourceModal, setShowMilestoneResourceModal] = useState(false);
  const [selectedMilestoneResource, setSelectedMilestoneResource] = useState<MilestoneResource | undefined>(undefined);

  // Note modal state
  const [showNoteModal, setShowNoteModal] = useState(false);
  const [selectedNote, setSelectedNote] = useState<Note | undefined>(undefined);
  const [showNoteViewModal, setShowNoteViewModal] = useState(false);
  const [noteForView, setNoteForView] = useState<Note | null>(null);

  // Detail view modals
  const [showMilestoneDetailModal, setShowMilestoneDetailModal] = useState(false);
  const [milestoneForDetail, setMilestoneForDetail] = useState<Milestone | null>(null);
  const [milestoneNotes, setMilestoneNotes] = useState<MilestoneNote[]>([]);
  const [showMilestoneNoteForm, setShowMilestoneNoteForm] = useState(false);

  const [showStakeholderDetailModal, setShowStakeholderDetailModal] = useState(false);
  const [stakeholderForDetail, setStakeholderForDetail] = useState<ProjectStakeholder | null>(null);
  const [stakeholderNotes, setStakeholderNotes] = useState<StakeholderNote[]>([]);
  const [showStakeholderNoteForm, setShowStakeholderNoteForm] = useState(false);

  useEffect(() => {
    loadProjectData();
  }, [projectId]);

  const loadProjectData = async () => {
    setLoading(true);
    try {
      const [projectData, milestonesData, stakeholdersData, resourcesData, peopleData, notesData, jiraUrl] = await Promise.all([
        ProjectService.getProject(projectId),
        ProjectService.getProjectMilestones(projectId),
        ProjectService.getProjectStakeholders(projectId),
        ProjectService.getProjectResources(projectId),
        PersonService.listPeople(),
        NoteService.getProjectNotes(projectId),
        ProjectService.getJiraUrl(),
      ]);

      setProject(projectData);
      setMilestones(milestonesData);
      setStakeholders(stakeholdersData);
      setProjectResources(resourcesData);
      setPeople(peopleData);
      setProjectNotes(notesData);
      setJiraBaseUrl(jiraUrl);

      // Load milestone resources for each milestone
      const milestoneResourcesMap = new Map<string, MilestoneResource[]>();
      await Promise.all(
        milestonesData.map(async (milestone) => {
          const resources = await ProjectService.getMilestoneResources(milestone.id);
          milestoneResourcesMap.set(milestone.id, resources);
        })
      );
      setMilestoneResources(milestoneResourcesMap);
    } catch (error) {
      message.error('Failed to load project details: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateMilestone = () => {
    setSelectedMilestone(undefined);
    setShowMilestoneModal(true);
  };

  const handleEditMilestone = (milestone: Milestone) => {
    setSelectedMilestone(milestone);
    setShowMilestoneModal(true);
  };

  const handleSaveMilestone = async () => {
    setShowMilestoneModal(false);
    setSelectedMilestone(undefined);
    await loadProjectData();
  };

  const handleCancelMilestone = () => {
    setShowMilestoneModal(false);
    setSelectedMilestone(undefined);
  };

  const handleDeleteMilestone = async (milestone: Milestone) => {
    Modal.confirm({
      title: 'Delete Milestone',
      content: `Are you sure you want to delete milestone "${milestone.name}"?`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await MilestoneService.deleteMilestone(milestone.id);
          message.success('Milestone deleted successfully');
          await loadProjectData();
        } catch (error) {
          message.error('Failed to delete milestone: ' + error);
        }
      },
    });
  };

  const handleViewMilestone = async (milestone: Milestone) => {
    try {
      const notes = await NoteService.getMilestoneNotes(milestone.id);
      setMilestoneForDetail(milestone);
      setMilestoneNotes(notes);
      setShowMilestoneDetailModal(true);
    } catch (error) {
      message.error('Failed to load milestone details: ' + error);
    }
  };

  const handleCreateStakeholder = () => {
    setSelectedStakeholder(undefined);
    setShowStakeholderModal(true);
  };

  const handleEditStakeholder = (stakeholder: ProjectStakeholder) => {
    setSelectedStakeholder(stakeholder);
    setShowStakeholderModal(true);
  };

  const handleSaveStakeholder = async (stakeholder: ProjectStakeholder) => {
    try {
      if (selectedStakeholder) {
        await ProjectService.updateStakeholder(projectId, stakeholder);
        message.success('Stakeholder updated successfully');
      } else {
        await ProjectService.addProjectStakeholder(projectId, stakeholder);
        message.success('Stakeholder added successfully');
      }
      setShowStakeholderModal(false);
      setSelectedStakeholder(undefined);
      await loadProjectData();
    } catch (error) {
      message.error('Failed to save stakeholder: ' + error);
    }
  };

  const handleCancelStakeholder = () => {
    setShowStakeholderModal(false);
    setSelectedStakeholder(undefined);
  };

  const handleDeleteStakeholder = async (stakeholder: ProjectStakeholder) => {
    Modal.confirm({
      title: 'Delete Stakeholder',
      content: `Are you sure you want to remove ${stakeholder.stakeholder_email} as a stakeholder?`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await ProjectService.removeStakeholder(projectId, stakeholder.stakeholder_email);
          message.success('Stakeholder removed successfully');
          await loadProjectData();
        } catch (error) {
          message.error('Failed to remove stakeholder: ' + error);
        }
      },
    });
  };

  const handleViewStakeholder = async (stakeholder: ProjectStakeholder) => {
    try {
      const notes = await NoteService.getStakeholderNotes(projectId, stakeholder.stakeholder_email);
      setStakeholderForDetail(stakeholder);
      setStakeholderNotes(notes);
      setShowStakeholderDetailModal(true);
    } catch (error) {
      message.error('Failed to load stakeholder details: ' + error);
    }
  };

  // Project Resource handlers
  const handleCreateProjectResource = () => {
    setSelectedProjectResource(undefined);
    setShowProjectResourceModal(true);
  };

  const handleEditProjectResource = (resource: ProjectResource) => {
    setSelectedProjectResource(resource);
    setShowProjectResourceModal(true);
  };

  const handleSaveProjectResource = async (resource: ProjectResource) => {
    try {
      if (selectedProjectResource) {
        await ProjectService.updateProjectResource(projectId, resource);
        message.success('Resource updated successfully');
      } else {
        await ProjectService.addProjectResource(projectId, resource);
        message.success('Resource added successfully');
      }
      setShowProjectResourceModal(false);
      setSelectedProjectResource(undefined);
      await loadProjectData();
    } catch (error) {
      message.error('Failed to save resource: ' + error);
    }
  };

  const handleCancelProjectResource = () => {
    setShowProjectResourceModal(false);
    setSelectedProjectResource(undefined);
  };

  const handleDeleteProjectResource = async (resource: ProjectResource) => {
    Modal.confirm({
      title: 'Delete Resource',
      content: `Are you sure you want to remove ${resource.person_email} from this project?`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await ProjectService.removeProjectResource(projectId, resource.person_email);
          message.success('Resource removed successfully');
          await loadProjectData();
        } catch (error) {
          message.error('Failed to remove resource: ' + error);
        }
      },
    });
  };

  // Milestone Resource handlers
  const handleCreateMilestoneResource = (milestoneId: string) => {
    setSelectedMilestone(milestones.find(m => m.id === milestoneId));
    setSelectedMilestoneResource(undefined);
    setShowMilestoneResourceModal(true);
  };

  const handleEditMilestoneResource = (milestoneId: string, resource: MilestoneResource) => {
    setSelectedMilestone(milestones.find(m => m.id === milestoneId));
    setSelectedMilestoneResource(resource);
    setShowMilestoneResourceModal(true);
  };

  const handleSaveMilestoneResource = async (resource: MilestoneResource) => {
    if (!selectedMilestone) return;
    try {
      if (selectedMilestoneResource) {
        await ProjectService.updateMilestoneResource(selectedMilestone.id, resource);
        message.success('Resource updated successfully');
      } else {
        await ProjectService.addMilestoneResource(selectedMilestone.id, resource);
        message.success('Resource added successfully');
      }
      setShowMilestoneResourceModal(false);
      setSelectedMilestoneResource(undefined);
      await loadProjectData();
    } catch (error) {
      message.error('Failed to save resource: ' + error);
    }
  };

  const handleCancelMilestoneResource = () => {
    setShowMilestoneResourceModal(false);
    setSelectedMilestoneResource(undefined);
  };

  const handleDeleteMilestoneResource = async (milestoneId: string, resource: MilestoneResource) => {
    Modal.confirm({
      title: 'Delete Resource',
      content: `Are you sure you want to remove ${resource.person_email} from this milestone?`,
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await ProjectService.removeMilestoneResource(milestoneId, resource.person_email);
          message.success('Resource removed successfully');
          await loadProjectData();
        } catch (error) {
          message.error('Failed to remove resource: ' + error);
        }
      },
    });
  };

  const handleCreateNote = () => {
    setSelectedNote(undefined);
    setShowNoteModal(true);
  };

  const handleViewNote = (note: Note) => {
    setNoteForView(note);
    setShowNoteViewModal(true);
  };

  const handleEditNote = (note: Note) => {
    setSelectedNote(note);
    setShowNoteViewModal(false);
    setShowNoteModal(true);
  };

  const handleSaveNote = async (title: string, body: string, noteId?: string) => {
    try {
      if (noteId) {
        // Editing existing note
        const existingNote = projectNotes.find(n => n.id === noteId);
        if (existingNote) {
          const updatedNote: ProjectNote = {
            ...existingNote,
            title,
            body,
            updated_at: new Date().toISOString(),
          };
          await NoteService.updateProjectNote(updatedNote);
          message.success('Note updated successfully');
        }
      } else {
        // Creating new note
        const now = new Date().toISOString();
        const note: ProjectNote = {
          id: crypto.randomUUID(),
          project_id: projectId,
          title,
          body,
          created_at: now,
          updated_at: now,
        };
        await NoteService.addProjectNote(note);
        message.success('Note added successfully');
      }
      setShowNoteModal(false);
      setSelectedNote(undefined);
      await loadProjectData();
    } catch (error) {
      message.error(`Failed to ${noteId ? 'update' : 'add'} note: ` + error);
    }
  };

  const handleCancelNote = () => {
    setShowNoteModal(false);
    setSelectedNote(undefined);
  };

  const handleDeleteNote = async (noteId: string) => {
    await NoteService.deleteProjectNote(noteId);
    await loadProjectData();
  };

  const handleDeleteNoteFromModal = async (noteId: string) => {
    Modal.confirm({
      title: 'Delete Note',
      content: 'Are you sure you want to delete this note?',
      okText: 'Delete',
      okType: 'danger',
      onOk: async () => {
        try {
          await NoteService.deleteProjectNote(noteId);
          message.success('Note deleted successfully');
          setShowNoteViewModal(false);
          setNoteForView(null);
          await loadProjectData();
        } catch (error) {
          message.error('Failed to delete note: ' + error);
        }
      },
    });
  };

  const handleSaveMilestoneNote = async (title: string, body: string) => {
    if (!milestoneForDetail) return;
    try {
      const now = new Date().toISOString();
      const note: MilestoneNote = {
        id: crypto.randomUUID(),
        milestone_id: milestoneForDetail.id,
        title,
        body,
        created_at: now,
        updated_at: now,
      };
      await NoteService.addMilestoneNote(note);
      message.success('Note added successfully');
      setShowMilestoneNoteForm(false);
      const notes = await NoteService.getMilestoneNotes(milestoneForDetail.id);
      setMilestoneNotes(notes);
    } catch (error) {
      message.error('Failed to add note: ' + error);
    }
  };

  const handleSaveStakeholderNote = async (title: string, body: string) => {
    if (!stakeholderForDetail) return;
    try {
      const now = new Date().toISOString();
      const note: StakeholderNote = {
        id: crypto.randomUUID(),
        project_id: projectId,
        stakeholder_email: stakeholderForDetail.stakeholder_email,
        title,
        body,
        created_at: now,
        updated_at: now,
      };
      await NoteService.addStakeholderNote(note);
      message.success('Note added successfully');
      setShowStakeholderNoteForm(false);
      const notes = await NoteService.getStakeholderNotes(projectId, stakeholderForDetail.stakeholder_email);
      setStakeholderNotes(notes);
    } catch (error) {
      message.error('Failed to add note: ' + error);
    }
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString();
  };

  const renderJiraLink = (ticketNumber?: string) => {
    if (!ticketNumber) return '-';
    return (
      <Link href={`${jiraBaseUrl}${ticketNumber}`} target="_blank">
        <LinkOutlined /> {ticketNumber}
      </Link>
    );
  };

  const milestoneColumns: ColumnsType<Milestone> = [
    {
      title: '#',
      dataIndex: 'number',
      key: 'number',
      sorter: (a, b) => a.number - b.number,
      width: 60,
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      width: 150,
    },
    {
      title: 'Description',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
      width: 200,
      render: (text) => text || '-',
    },
    {
      title: 'Technical Lead',
      dataIndex: 'technical_lead',
      key: 'technical_lead',
      width: 150,
      render: (email) => email || '-',
    },
    {
      title: 'Due Date',
      dataIndex: 'due_date',
      key: 'due_date',
      width: 120,
      render: formatDate,
    },
    {
      title: 'Jira Epic',
      dataIndex: 'jira_epic',
      key: 'jira_epic',
      width: 120,
      render: renderJiraLink,
    },
    {
      title: 'Design Doc',
      dataIndex: 'design_doc_url',
      key: 'design_doc_url',
      width: 120,
      render: (url) => url ? (
        <Link href={url} target="_blank">
          <LinkOutlined /> View Doc
        </Link>
      ) : '-',
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 200,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EyeOutlined />}
            onClick={() => handleViewMilestone(record)}
          >
            View
          </Button>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEditMilestone(record)}
          >
            Edit
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDeleteMilestone(record)}
          >
            Delete
          </Button>
        </Space>
      ),
    },
  ];

  const getPersonName = (email: string) => {
    const person = people.find(p => p.email === email);
    return person?.name || email;
  };

  const stakeholderColumns: ColumnsType<ProjectStakeholder> = [
    {
      title: 'Name',
      dataIndex: 'stakeholder_email',
      key: 'name',
      render: (email) => getPersonName(email),
    },
    {
      title: 'Email',
      dataIndex: 'stakeholder_email',
      key: 'stakeholder_email',
    },
    {
      title: 'Role',
      dataIndex: 'role',
      key: 'role',
      render: (text) => text || '-',
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 200,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EyeOutlined />}
            onClick={() => handleViewStakeholder(record)}
          >
            View
          </Button>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEditStakeholder(record)}
          >
            Edit
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDeleteStakeholder(record)}
          >
            Delete
          </Button>
        </Space>
      ),
    },
  ];

  const projectResourceColumns: ColumnsType<ProjectResource> = [
    {
      title: 'Name',
      dataIndex: 'person_email',
      key: 'name',
      render: (email) => getPersonName(email),
    },
    {
      title: 'Email',
      dataIndex: 'person_email',
      key: 'person_email',
    },
    {
      title: 'Role',
      dataIndex: 'role',
      key: 'role',
      render: (text) => text || '-',
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 150,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEditProjectResource(record)}
          >
            Edit
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDeleteProjectResource(record)}
          >
            Delete
          </Button>
        </Space>
      ),
    },
  ];

  const getMilestoneResourceColumns = (milestoneId: string): ColumnsType<MilestoneResource> => [
    {
      title: 'Name',
      dataIndex: 'person_email',
      key: 'name',
      render: (email) => getPersonName(email),
    },
    {
      title: 'Email',
      dataIndex: 'person_email',
      key: 'person_email',
    },
    {
      title: 'Role',
      dataIndex: 'role',
      key: 'role',
      render: (text) => text || '-',
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 150,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={() => handleEditMilestoneResource(milestoneId, record)}
          >
            Edit
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDeleteMilestoneResource(milestoneId, record)}
          >
            Delete
          </Button>
        </Space>
      ),
    },
  ];

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '50px' }}>
        <Spin size="large" />
      </div>
    );
  }

  if (!project) {
    return <div>Project not found</div>;
  }

  return (
    <div>
      <Space style={{ marginBottom: 16 }}>
        <Button icon={<ArrowLeftOutlined />} onClick={onBack}>
          Back to Projects
        </Button>
        <Button type="primary" icon={<EditOutlined />} onClick={onEdit}>
          Edit Project
        </Button>
      </Space>

      <Title level={2}>{project.name}</Title>

      <Card title="Project Details" style={{ marginBottom: 16 }}>
        <Descriptions column={2}>
          <Descriptions.Item label="Description" span={2}>
            {project.description || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Requirements Owner">
            {project.requirements_owner || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Technical Lead">
            {project.technical_lead || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Manager">
            {project.manager || '-'}
          </Descriptions.Item>
          <Descriptions.Item label="Due Date">
            {formatDate(project.due_date)}
          </Descriptions.Item>
          <Descriptions.Item label="Jira Initiative">
            {renderJiraLink(project.jira_initiative)}
          </Descriptions.Item>
          <Descriptions.Item label="Created">
            {formatDate(project.created_at)}
          </Descriptions.Item>
          <Descriptions.Item label="Last Updated">
            {formatDate(project.updated_at)}
          </Descriptions.Item>
        </Descriptions>
      </Card>

      <Card
        title="Milestones"
        style={{ marginBottom: 16 }}
        extra={
          <Button
            type="primary"
            size="small"
            icon={<PlusOutlined />}
            onClick={handleCreateMilestone}
          >
            Add Milestone
          </Button>
        }
      >
        <Table
          columns={milestoneColumns}
          dataSource={milestones}
          rowKey="id"
          pagination={false}
          scroll={{ x: 1100 }}
          locale={{ emptyText: 'No milestones yet' }}
        />
      </Card>

      <Card
        title="Stakeholders"
        style={{ marginBottom: 16 }}
        extra={
          <Button
            type="primary"
            size="small"
            icon={<PlusOutlined />}
            onClick={handleCreateStakeholder}
          >
            Add Stakeholder
          </Button>
        }
      >
        <Table
          columns={stakeholderColumns}
          dataSource={stakeholders}
          rowKey="stakeholder_email"
          pagination={false}
          locale={{ emptyText: 'No stakeholders yet' }}
        />
      </Card>

      <Card
        title="Resources"
        style={{ marginBottom: 16 }}
        extra={
          <Button
            type="primary"
            size="small"
            icon={<PlusOutlined />}
            onClick={handleCreateProjectResource}
          >
            Add Resource
          </Button>
        }
      >
        <Table
          columns={projectResourceColumns}
          dataSource={projectResources}
          rowKey="person_email"
          pagination={false}
          locale={{ emptyText: 'No resources assigned yet' }}
        />
      </Card>

      <Card
        title="Notes"
        extra={
          <Button
            type="primary"
            size="small"
            icon={<PlusOutlined />}
            onClick={handleCreateNote}
          >
            Add Note
          </Button>
        }
      >
        <NoteList
          notes={projectNotes}
          onView={handleViewNote}
          onDelete={handleDeleteNote}
          loading={loading}
        />
      </Card>

      <Modal
        title={selectedMilestone ? 'Edit Milestone' : 'New Milestone'}
        open={showMilestoneModal}
        onCancel={handleCancelMilestone}
        footer={null}
        width={700}
        destroyOnClose
      >
        <MilestoneForm
          projectId={projectId}
          milestone={selectedMilestone}
          onSave={handleSaveMilestone}
          onCancel={handleCancelMilestone}
          isModal
        />
      </Modal>

      <Modal
        title={selectedStakeholder ? 'Edit Stakeholder' : 'Add Stakeholder'}
        open={showStakeholderModal}
        onCancel={handleCancelStakeholder}
        footer={null}
        width={600}
        destroyOnClose
      >
        <StakeholderForm
          projectId={projectId}
          stakeholder={selectedStakeholder}
          onSave={handleSaveStakeholder}
          onCancel={handleCancelStakeholder}
          isModal
        />
      </Modal>

      <Modal
        title={selectedProjectResource ? 'Edit Resource' : 'Add Resource'}
        open={showProjectResourceModal}
        onCancel={handleCancelProjectResource}
        footer={null}
        destroyOnClose
      >
        <ProjectResourceForm
          projectId={projectId}
          resource={selectedProjectResource}
          onSave={handleSaveProjectResource}
          onCancel={handleCancelProjectResource}
          isModal
        />
      </Modal>

      <Modal
        title={selectedMilestoneResource ? 'Edit Resource' : 'Add Resource'}
        open={showMilestoneResourceModal}
        onCancel={handleCancelMilestoneResource}
        footer={null}
        destroyOnClose
      >
        {selectedMilestone && (
          <MilestoneResourceForm
            milestoneId={selectedMilestone.id}
            resource={selectedMilestoneResource}
            onSave={handleSaveMilestoneResource}
            onCancel={handleCancelMilestoneResource}
            isModal
          />
        )}
      </Modal>

      <Modal
        title={selectedNote ? 'Edit Note' : 'Add Note'}
        open={showNoteModal}
        onCancel={handleCancelNote}
        footer={null}
        width={600}
        destroyOnClose
      >
        <NoteForm
          note={selectedNote}
          onSave={handleSaveNote}
          onCancel={handleCancelNote}
        />
      </Modal>

      <NoteViewModal
        note={noteForView}
        open={showNoteViewModal}
        onClose={() => {
          setShowNoteViewModal(false);
          setNoteForView(null);
        }}
        onEdit={handleEditNote}
        onDelete={handleDeleteNoteFromModal}
      />

      <Modal
        title={milestoneForDetail ? `Milestone: ${milestoneForDetail.name}` : 'Milestone Details'}
        open={showMilestoneDetailModal}
        onCancel={() => setShowMilestoneDetailModal(false)}
        footer={null}
        width={800}
      >
        {milestoneForDetail && (
          <div>
            <Descriptions column={1} bordered style={{ marginBottom: 16 }}>
              <Descriptions.Item label="Name">{milestoneForDetail.name}</Descriptions.Item>
              <Descriptions.Item label="Description">
                {milestoneForDetail.description || '-'}
              </Descriptions.Item>
              <Descriptions.Item label="Technical Lead">
                {milestoneForDetail.technical_lead || '-'}
              </Descriptions.Item>
              <Descriptions.Item label="Due Date">
                {formatDate(milestoneForDetail.due_date)}
              </Descriptions.Item>
              <Descriptions.Item label="Jira Epic">
                {renderJiraLink(milestoneForDetail.jira_epic)}
              </Descriptions.Item>
              <Descriptions.Item label="Design Doc">
                {milestoneForDetail.design_doc_url ? (
                  <Link href={milestoneForDetail.design_doc_url} target="_blank">
                    <LinkOutlined /> View Doc
                  </Link>
                ) : '-'}
              </Descriptions.Item>
            </Descriptions>
            <Card
              title="Resources"
              size="small"
              style={{ marginBottom: 16 }}
              extra={
                <Button
                  type="primary"
                  size="small"
                  icon={<PlusOutlined />}
                  onClick={() => handleCreateMilestoneResource(milestoneForDetail.id)}
                >
                  Add Resource
                </Button>
              }
            >
              <Table
                columns={getMilestoneResourceColumns(milestoneForDetail.id)}
                dataSource={milestoneResources.get(milestoneForDetail.id) || []}
                rowKey="person_email"
                pagination={false}
                size="small"
                locale={{ emptyText: 'No resources assigned yet' }}
              />
            </Card>
            <Card
              title="Notes"
              size="small"
              extra={
                <Button
                  type="primary"
                  size="small"
                  icon={<PlusOutlined />}
                  onClick={() => setShowMilestoneNoteForm(true)}
                >
                  Add Note
                </Button>
              }
            >
              {showMilestoneNoteForm ? (
                <div style={{ marginBottom: 16 }}>
                  <NoteForm
                    onSave={handleSaveMilestoneNote}
                    onCancel={() => setShowMilestoneNoteForm(false)}
                  />
                </div>
              ) : null}
              <NoteList
                notes={milestoneNotes}
                onView={(note) => {
                  Modal.info({
                    title: note.title,
                    content: note.body,
                    width: 600,
                  });
                }}
                onDelete={async (id) => {
                  await NoteService.deleteMilestoneNote(id);
                  const notes = await NoteService.getMilestoneNotes(milestoneForDetail.id);
                  setMilestoneNotes(notes);
                }}
              />
            </Card>
          </div>
        )}
      </Modal>

      <Modal
        title={stakeholderForDetail ? `Stakeholder: ${getPersonName(stakeholderForDetail.stakeholder_email)}` : 'Stakeholder Details'}
        open={showStakeholderDetailModal}
        onCancel={() => setShowStakeholderDetailModal(false)}
        footer={null}
        width={800}
      >
        {stakeholderForDetail && (
          <div>
            <Descriptions column={1} bordered style={{ marginBottom: 16 }}>
              <Descriptions.Item label="Name">
                {getPersonName(stakeholderForDetail.stakeholder_email)}
              </Descriptions.Item>
              <Descriptions.Item label="Email">
                {stakeholderForDetail.stakeholder_email}
              </Descriptions.Item>
              <Descriptions.Item label="Role">
                {stakeholderForDetail.role || '-'}
              </Descriptions.Item>
            </Descriptions>
            <Card
              title="Notes"
              size="small"
              extra={
                <Button
                  type="primary"
                  size="small"
                  icon={<PlusOutlined />}
                  onClick={() => setShowStakeholderNoteForm(true)}
                >
                  Add Note
                </Button>
              }
            >
              {showStakeholderNoteForm ? (
                <div style={{ marginBottom: 16 }}>
                  <NoteForm
                    onSave={handleSaveStakeholderNote}
                    onCancel={() => setShowStakeholderNoteForm(false)}
                  />
                </div>
              ) : null}
              <NoteList
                notes={stakeholderNotes}
                onView={(note) => {
                  Modal.info({
                    title: note.title,
                    content: note.body,
                    width: 600,
                  });
                }}
                onDelete={async (id) => {
                  await NoteService.deleteStakeholderNote(id);
                  const notes = await NoteService.getStakeholderNotes(projectId, stakeholderForDetail.stakeholder_email);
                  setStakeholderNotes(notes);
                }}
              />
            </Card>
          </div>
        )}
      </Modal>
    </div>
  );
};
