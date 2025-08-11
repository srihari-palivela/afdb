import { Button, Group, Paper, Stack, Table, TextInput, Title } from '@mantine/core';
import { useEffect, useState } from 'react';
import { api } from '../api/client';

interface Contract { producer: string; schema_hash: string; pii_fields: string[] }

export default function Contracts() {
  const [rows, setRows] = useState<Contract[]>([]);
  const [producer, setProducer] = useState('confluence');
  const [hash, setHash] = useState('deadbeef');
  const [pii, setPii] = useState('email');

  const load = async () => {
    const { data } = await api.get('/contracts');
    setRows(data);
  };

  const register = async () => {
    await api.post('/contracts', { contract: { producer, schema_hash: hash, pii_fields: pii.split(',').map((s) => s.trim()) } });
    await load();
  };

  useEffect(() => { load(); }, []);

  return (
    <Paper p="md" radius="md" withBorder>
      <Stack>
        <Title order={4}>Data Contracts</Title>
        <Group grow>
          <TextInput label="Producer" value={producer} onChange={(e) => setProducer(e.currentTarget.value)} />
          <TextInput label="Schema hash" value={hash} onChange={(e) => setHash(e.currentTarget.value)} />
          <TextInput label="PII fields (comma-separated)" value={pii} onChange={(e) => setPii(e.currentTarget.value)} />
          <Button onClick={register}>Register</Button>
        </Group>
        <Table striped stickyHeader>
          <Table.Thead>
            <Table.Tr><Table.Th>Producer</Table.Th><Table.Th>Schema</Table.Th><Table.Th>PII</Table.Th></Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {rows.map((r, i) => (
              <Table.Tr key={i}><Table.Td>{r.producer}</Table.Td><Table.Td>{r.schema_hash}</Table.Td><Table.Td>{r.pii_fields.join(', ')}</Table.Td></Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      </Stack>
    </Paper>
  );
}
