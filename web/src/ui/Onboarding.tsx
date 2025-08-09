import { Button, Group, Paper, Select, Stack, Text, TextInput, Title } from '@mantine/core';
import { useState } from 'react';
import axios from 'axios';
import { notifications } from '@mantine/notifications';

export default function Onboarding() {
  const [company, setCompany] = useState('Acme Corp');
  const [rootUnit, setRootUnit] = useState('Manufacturing/QA');

  const submit = async () => {
    // Placeholder calls: contracts + role to create session for demo
    await axios.post(import.meta.env.VITE_API_BASE_URL + '/contracts', { contract: { producer: 'confluence', schema_hash: 'deadbeef', pii_fields: ['email'] } });
    const { data } = await axios.post(import.meta.env.VITE_API_BASE_URL + '/assume_role', { person_id: 'u1', roles: ['QA-Inspector'], scope_ids: [1] });
    notifications.show({ title: 'Onboarding complete', message: `Session ${data.session_id} created`, color: 'green' });
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
