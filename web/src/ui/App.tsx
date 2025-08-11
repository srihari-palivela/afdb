import { AppShell, Button, Container, Group, Tabs, Title, Badge } from '@mantine/core';
import Onboarding from './Onboarding';
import Ingestion from './Ingestion';
import Query from './Query';
import RoleAssume from './RoleAssume';
import Contracts from './Contracts';
import OrgUnits from './OrgUnits';
import Taxonomy from './Taxonomy';
import { useSession } from '../store/session';

export default function App() {
  const { sessionId } = useSession();
  return (
    <AppShell header={{ height: 64 }} padding="md">
      <AppShell.Header>
        <Group justify="space-between" px="md" h="100%">
          <Group gap="xs">
            <Title order={3}>AFDB Console</Title>
            <Badge variant="light" color="gray">Enterprise</Badge>
          </Group>
          <Group gap="xs">
            {sessionId ? <Badge color="green" variant="light">Session</Badge> : <Badge color="yellow" variant="light">No session</Badge>}
            <Button component="a" href="https://github.com/srihari-palivela/afdb" variant="light">GitHub</Button>
          </Group>
        </Group>
      </AppShell.Header>
      <AppShell.Main>
        <Container size="lg">
          <Tabs defaultValue="onboarding" variant="outline" radius="md">
            <Tabs.List>
              <Tabs.Tab value="onboarding">Onboarding</Tabs.Tab>
              <Tabs.Tab value="ingestion">Ingestion</Tabs.Tab>
              <Tabs.Tab value="query">Query</Tabs.Tab>
              <Tabs.Tab value="roles">Roles</Tabs.Tab>
              <Tabs.Tab value="contracts">Contracts</Tabs.Tab>
              <Tabs.Tab value="org">Org Units</Tabs.Tab>
              <Tabs.Tab value="taxonomy">Taxonomy</Tabs.Tab>
            </Tabs.List>
            <Tabs.Panel value="onboarding" pt="md"><Onboarding /></Tabs.Panel>
            <Tabs.Panel value="ingestion" pt="md"><Ingestion /></Tabs.Panel>
            <Tabs.Panel value="query" pt="md"><Query /></Tabs.Panel>
            <Tabs.Panel value="roles" pt="md"><RoleAssume /></Tabs.Panel>
            <Tabs.Panel value="contracts" pt="md"><Contracts /></Tabs.Panel>
            <Tabs.Panel value="org" pt="md"><OrgUnits /></Tabs.Panel>
            <Tabs.Panel value="taxonomy" pt="md"><Taxonomy /></Tabs.Panel>
          </Tabs>
        </Container>
      </AppShell.Main>
    </AppShell>
  );
}
