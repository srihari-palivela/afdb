import { Button, Group, Paper, Stack, Text, TextInput, Title } from '@mantine/core';
import { useState } from 'react';

export default function Onboarding() {
  const [company, setCompany] = useState('Acme Corp');
  const [rootUnit, setRootUnit] = useState('Manufacturing/QA');

  const submit = () => {
    // Placeholder: would POST to backend org/taxonomy setup endpoints
    alert(`Onboarded ${company} with root ${rootUnit}`);
  };

  return (
    <Paper p="md" radius="md" withBorder>
      <Stack>
        <Title order={4}>First-run setup</Title>
        <TextInput label="Company name" value={company} onChange={(e) => setCompany(e.currentTarget.value)} />
        <TextInput label="Root org unit" value={rootUnit} onChange={(e) => setRootUnit(e.currentTarget.value)} />
        <Group justify="flex-end">
          <Button onClick={submit}>Save</Button>
        </Group>
        <Text c="dimmed" size="sm">This will configure org units, taxonomy, default spaces and policies.</Text>
      </Stack>
    </Paper>
  );
}
