import { Button, Group, MultiSelect, Paper, Stack, TextInput, Title } from '@mantine/core';
import axios from 'axios';
import { useState } from 'react';
import { useSession } from '../store/session';
import { notifications } from '@mantine/notifications';

export default function RoleAssume() {
  const [personId, setPersonId] = useState('u1');
  const [roles, setRoles] = useState<string[]>(['QA-Inspector']);
  const [scopes, setScopes] = useState<string[]>(['1']);
  const { setSession } = useSession();

  const assume = async () => {
    const { data } = await axios.post('http://localhost:8090/assume_role', { person_id: personId, roles, scope_ids: scopes.map((s) => parseInt(s)) });
    setSession(data.session_id);
    notifications.show({ title: 'Role assumed', message: `Session ${data.session_id}`, color: 'green' });
  };

  return (
    <Paper p="md" radius="md" withBorder>
      <Stack>
        <Title order={4}>Assume Role</Title>
        <Group grow>
          <TextInput label="Person ID" value={personId} onChange={(e) => setPersonId(e.currentTarget.value)} />
          <MultiSelect label="Roles" data={[ 'QA-Inspector', 'QA-Lead', 'Support-Agent' ]} value={roles} onChange={setRoles} searchable />
          <MultiSelect label="Org scope IDs" data={[ '1', '2', '3' ]} value={scopes} onChange={setScopes} searchable />
        </Group>
        <Group justify="flex-end"><Button onClick={assume}>Assume</Button></Group>
      </Stack>
    </Paper>
  );
}
