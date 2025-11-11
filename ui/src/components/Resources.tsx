/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */

import { useState, useEffect, useMemo } from 'react';
import { Card, Select, Space, Spin, message, Typography, Button, Tag } from 'antd';
import { ZoomInOutlined } from '@ant-design/icons';
import { ProjectService } from '../services/projectService';
import { PersonService } from '../services/personService';
import { TeamService } from '../services/teamService';
import type { Project, Person, Team, Milestone, ProjectResource, MilestoneResource } from '../types';
import './Resources.css';

const { Title, Text } = Typography;

type ZoomLevel = 'day' | 'week' | 'month';

interface Assignment {
  personEmail: string;
  projectId: string;
  projectName: string;
  milestoneId?: string;
  milestoneName?: string;
  startDate: Date;
  endDate: Date;
  type: 'project' | 'milestone';
}

const PROJECT_COLORS = [
  '#1890ff', '#52c41a', '#fa8c16', '#eb2f96', '#13c2c2',
  '#722ed1', '#f5222d', '#faad14', '#2f54eb', '#fa541c',
];

export const Resources: React.FC = () => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [people, setPeople] = useState<Person[]>([]);
  const [teams, setTeams] = useState<Team[]>([]);
  const [milestones, setMilestones] = useState<Map<string, Milestone[]>>(new Map());
  const [projectResources, setProjectResources] = useState<Map<string, ProjectResource[]>>(new Map());
  const [milestoneResources, setMilestoneResources] = useState<Map<string, MilestoneResource[]>>(new Map());
  const [loading, setLoading] = useState(false);
  const [selectedTeam, setSelectedTeam] = useState<string | undefined>(undefined);
  const [selectedProject, setSelectedProject] = useState<string | undefined>(undefined);
  const [zoomLevel, setZoomLevel] = useState<ZoomLevel>('week');

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [projectsData, peopleData, teamsData] = await Promise.all([
        ProjectService.listProjects(),
        PersonService.listPeople(),
        TeamService.listTeams(),
      ]);

      setProjects(projectsData);
      setPeople(peopleData);
      setTeams(teamsData);

      // Load milestones and resources for each project
      const milestonesMap = new Map<string, Milestone[]>();
      const projectResourcesMap = new Map<string, ProjectResource[]>();
      const milestoneResourcesMap = new Map<string, MilestoneResource[]>();

      await Promise.all(
        projectsData.map(async (project) => {
          const [projectMilestones, resources] = await Promise.all([
            ProjectService.getProjectMilestones(project.id),
            ProjectService.getProjectResources(project.id),
          ]);

          milestonesMap.set(project.id, projectMilestones);
          projectResourcesMap.set(project.id, resources);

          // Load milestone resources
          await Promise.all(
            projectMilestones.map(async (milestone) => {
              const milestoneRes = await ProjectService.getMilestoneResources(milestone.id);
              milestoneResourcesMap.set(milestone.id, milestoneRes);
            })
          );
        })
      );

      setMilestones(milestonesMap);
      setProjectResources(projectResourcesMap);
      setMilestoneResources(milestoneResourcesMap);
    } catch (error) {
      message.error('Failed to load data: ' + error);
    } finally {
      setLoading(false);
    }
  };

  // Get color for a project
  const getProjectColor = (projectId: string): string => {
    const index = projects.findIndex((p) => p.id === projectId);
    return PROJECT_COLORS[index % PROJECT_COLORS.length];
  };

  // Generate assignments from projects and milestones
  const assignments = useMemo((): Assignment[] => {
    const result: Assignment[] = [];

    // Calculate timeline bounds
    const now = new Date();
    const timelineStart = new Date(now);
    timelineStart.setDate(1);
    timelineStart.setHours(0, 0, 0, 0);
    const timelineEnd = new Date(timelineStart);
    timelineEnd.setMonth(timelineEnd.getMonth() + 6);

    projects.forEach((project) => {
      // Project-level assignments from resources
      const resources = projectResources.get(project.id) || [];
      if (resources.length > 0) {
        // Use start_date if available, otherwise beginning of timeline
        const startDate = project.start_date ? new Date(project.start_date) : new Date(timelineStart);
        // Use due_date if available, otherwise end of timeline
        const endDate = project.due_date ? new Date(project.due_date) : new Date(timelineEnd);

        resources.forEach((resource) => {
          result.push({
            personEmail: resource.person_email,
            projectId: project.id,
            projectName: project.name,
            startDate,
            endDate,
            type: 'project',
          });
        });
      }

      // Milestone-level assignments from resources
      const projectMilestones = milestones.get(project.id) || [];
      projectMilestones.forEach((milestone) => {
        const milestoneRes = milestoneResources.get(milestone.id) || [];
        if (milestoneRes.length > 0) {
          // Use start_date if available, otherwise beginning of timeline
          const startDate = milestone.start_date ? new Date(milestone.start_date) : new Date(timelineStart);
          // Use due_date if available, otherwise end of timeline
          const endDate = milestone.due_date ? new Date(milestone.due_date) : new Date(timelineEnd);

          milestoneRes.forEach((resource) => {
            result.push({
              personEmail: resource.person_email,
              projectId: project.id,
              projectName: project.name,
              milestoneId: milestone.id,
              milestoneName: milestone.name,
              startDate,
              endDate,
              type: 'milestone',
            });
          });
        }
      });
    });

    return result;
  }, [projects, milestones, projectResources, milestoneResources]);

  // Filter people based on selected team
  const filteredPeople = useMemo(() => {
    if (!selectedTeam) return people;
    return people.filter((person) => person.team === selectedTeam);
  }, [people, selectedTeam]);

  // Filter assignments based on selected project
  const filteredAssignments = useMemo(() => {
    if (!selectedProject) return assignments;
    return assignments.filter((assignment) => assignment.projectId === selectedProject);
  }, [assignments, selectedProject]);

  // Generate time periods based on zoom level
  const timePeriods = useMemo(() => {
    const periods: Date[] = [];
    const now = new Date();
    const start = new Date(now);
    // Start at the beginning of the current month
    start.setDate(1);
    start.setHours(0, 0, 0, 0);
    const end = new Date(start);
    end.setMonth(end.getMonth() + 6); // End 6 months from start

    let current = new Date(start);

    while (current < end) {
      periods.push(new Date(current));

      if (zoomLevel === 'day') {
        current.setDate(current.getDate() + 1);
      } else if (zoomLevel === 'week') {
        current.setDate(current.getDate() + 7);
      } else {
        current.setMonth(current.getMonth() + 1);
      }
    }

    return periods;
  }, [zoomLevel]);

  // Format date for column header
  const formatPeriodHeader = (date: Date): string => {
    if (zoomLevel === 'day') {
      return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    } else if (zoomLevel === 'week') {
      const weekEnd = new Date(date);
      weekEnd.setDate(weekEnd.getDate() + 6);
      return `${date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} - ${weekEnd.toLocaleDateString('en-US', { day: 'numeric' })}`;
    } else {
      return date.toLocaleDateString('en-US', { month: 'short', year: 'numeric' });
    }
  };

  // Get period duration in days
  const getPeriodDuration = (): number => {
    if (zoomLevel === 'day') return 1;
    if (zoomLevel === 'week') return 7;
    return 30; // Approximate month
  };

  // Check if assignment overlaps with time period
  const assignmentOverlapsPeriod = (assignment: Assignment, period: Date): boolean => {
    const periodEnd = new Date(period);
    periodEnd.setDate(periodEnd.getDate() + getPeriodDuration());

    return assignment.startDate <= periodEnd && assignment.endDate >= period;
  };

  // Get assignments for a person in a specific time period
  const getPersonAssignments = (personEmail: string, period: Date): Assignment[] => {
    return filteredAssignments.filter(
      (assignment) =>
        assignment.personEmail === personEmail && assignmentOverlapsPeriod(assignment, period)
    );
  };

  const cycleZoomLevel = () => {
    const levels: ZoomLevel[] = ['day', 'week', 'month'];
    const currentIndex = levels.indexOf(zoomLevel);
    const nextIndex = (currentIndex + 1) % levels.length;
    setZoomLevel(levels[nextIndex]);
  };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: '50px' }}>
        <Spin size="large" />
      </div>
    );
  }

  return (
    <div className="resources-container">
      <Card>
        <div style={{ marginBottom: 16 }}>
          <Space direction="vertical" style={{ width: '100%' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <Title level={4} style={{ margin: 0 }}>
                Resource Allocation Timeline
              </Title>
              <Space>
                <Button icon={<ZoomInOutlined />} onClick={cycleZoomLevel}>
                  Zoom: {zoomLevel.charAt(0).toUpperCase() + zoomLevel.slice(1)}
                </Button>
              </Space>
            </div>
            <Space>
              <Text>Filter by Team:</Text>
              <Select
                style={{ width: 200 }}
                placeholder="All Teams"
                allowClear
                value={selectedTeam}
                onChange={setSelectedTeam}
              >
                {teams.map((team) => (
                  <Select.Option key={team.name} value={team.name}>
                    {team.name}
                  </Select.Option>
                ))}
              </Select>
              <Text>Filter by Project:</Text>
              <Select
                style={{ width: 200 }}
                placeholder="All Projects"
                allowClear
                value={selectedProject}
                onChange={setSelectedProject}
              >
                {projects.map((project) => (
                  <Select.Option key={project.id} value={project.id}>
                    {project.name}
                  </Select.Option>
                ))}
              </Select>
            </Space>
          </Space>
        </div>

        <div className="swimlane-container">
          <div className="swimlane-wrapper">
            {/* Header row */}
            <div className="swimlane-row swimlane-header">
              <div className="swimlane-person-cell swimlane-cell">Person</div>
              {timePeriods.map((period, index) => (
                <div key={index} className="swimlane-time-cell swimlane-cell">
                  {formatPeriodHeader(period)}
                </div>
              ))}
            </div>

            {/* Person rows */}
            {filteredPeople.map((person) => (
              <div key={person.email} className="swimlane-row">
                <div className="swimlane-person-cell swimlane-cell">
                  <div>
                    <div style={{ fontWeight: 500 }}>{person.name}</div>
                    {person.team && <Text type="secondary" style={{ fontSize: 12 }}>{person.team}</Text>}
                  </div>
                </div>
                {timePeriods.map((period, periodIndex) => {
                  const personAssignments = getPersonAssignments(person.email, period);
                  return (
                    <div key={periodIndex} className="swimlane-time-cell swimlane-cell">
                      {personAssignments.map((assignment, assignmentIndex) => (
                        <div
                          key={assignmentIndex}
                          className="assignment-block"
                          style={{ backgroundColor: getProjectColor(assignment.projectId) }}
                          title={`${assignment.projectName}${assignment.milestoneName ? ` - ${assignment.milestoneName}` : ''}`}
                        >
                          <div className="assignment-text">
                            {assignment.milestoneName || assignment.projectName}
                          </div>
                        </div>
                      ))}
                    </div>
                  );
                })}
              </div>
            ))}
          </div>
        </div>

        {/* Legend */}
        <div style={{ marginTop: 24 }}>
          <Text strong>Projects:</Text>
          <div style={{ marginTop: 8, display: 'flex', flexWrap: 'wrap', gap: 8 }}>
            {projects
              .filter((project) => !selectedProject || project.id === selectedProject)
              .map((project) => (
                <Tag key={project.id} color={getProjectColor(project.id)}>
                  {project.name}
                </Tag>
              ))}
          </div>
        </div>
      </Card>
    </div>
  );
};
