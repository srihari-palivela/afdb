import { Button, Chip, Chips, Group, Paper, Stack, TextInput, Title } from '@mantine/core';
import { useEffect, useState } from 'react';
import { api } from '../api/client';

export default function Taxonomy() {
  const [paths, setPaths] = useState<string[]>([]);
  const [path, setPath] = useState('Manufacturing/QA/PPAP');

  const load = async () => {
    const { data } = await api.get<string[]>('/taxonomy/paths');
    setPaths(data);
  };

  useEffect(() => { load(); }, []);

  const add = async () => {
    await api.post('/taxonomy/paths', { path });
    setPath('');
    await load();
  };

  return (
    <Paper p="md" radius="md" withBorder>
      <Stack>
        <Title order={4}>Taxonomy</Title>
        <Group grow>
          <TextInput label="Add path" value={path} placeholder="e.g. Support/Tickets/Refunds" onChange={(e) => setPath(e.currentTarget.value)} />
          <Button onClick={add}>Add</Button>
        </Group>
        <Chips multiple value={paths} onChange={() => {}} readOnly>
          {paths.map((p) => (
            <Chip key={p} value={p}>{p}</Chip>
          ))}
        </Chips>
      </Stack>
    </Paper>
  );
}
