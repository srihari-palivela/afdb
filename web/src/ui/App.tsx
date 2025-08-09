import { AppShell, Button, Container, Group, Tabs, Title } from '@mantine/core';
import Onboarding from './Onboarding';
import Ingestion from './Ingestion';
import Query from './Query';

export default function App() {
  return (
    <AppShell header={{ height: 60 }} padding="md">
      <AppShell.Header>
        <Group justify="space-between" px="md" h="100%">
          <Title order={3}>AFDB Console</Title>
          <Group>
            <Button component="a" href="https://github.com/srihari-palivela/afdb" variant="light">GitHub</Button>
          </Group>
        </Group>
      </AppShell.Header>
      <AppShell.Main>
        <Container size="lg">
          <Tabs defaultValue="onboarding">
            <Tabs.List>
              <Tabs.Tab value="onboarding">Onboarding</Tabs.Tab>
              <Tabs.Tab value="ingestion">Ingestion</Tabs.Tab>
              <Tabs.Tab value="query">Query</Tabs.Tab>
            </Tabs.List>
            <Tabs.Panel value="onboarding" pt="md"><Onboarding /></Tabs.Panel>
            <Tabs.Panel value="ingestion" pt="md"><Ingestion /></Tabs.Panel>
            <Tabs.Panel value="query" pt="md"><Query /></Tabs.Panel>
          </Tabs>
        </Container>
      </AppShell.Main>
    </AppShell>
  );
}
