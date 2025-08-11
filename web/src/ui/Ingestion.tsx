import { Button, FileInput, Group, Paper, Stack, TextInput, Title } from '@mantine/core';
import { useState } from 'react';
import { notifications } from '@mantine/notifications';
import { useSession } from '../store/session';
import { api } from '../api/client';

export default function Ingestion() {
  const [sourceApp, setSourceApp] = useState('confluence');
  const [orgHint, setOrgHint] = useState('Manufacturing/QA');
  const [file, setFile] = useState<File | null>(null);

  const { sessionId } = useSession();

  const upload = async () => {
    if (!file) return;
    const text = await file.text();
    const body = {
      manifest: {
        source_app: sourceApp,
        org_unit_hint: orgHint,
      },
      artifacts: [
        { id: file.name, text },
      ],
    };
    await api.post('/ingest', body);
    notifications.show({ title: 'Ingestion', message: 'Ingested successfully', color: 'green' });
  };

  return (
    <Paper p="md" radius="md" withBorder>
      <Stack>
        <Title order={4}>Upload documents</Title>
        <Group grow>
          <TextInput label="Source app" value={sourceApp} onChange={(e) => setSourceApp(e.currentTarget.value)} />
          <TextInput label="Org unit hint" value={orgHint} onChange={(e) => setOrgHint(e.currentTarget.value)} />
        </Group>
        <FileInput label="File" placeholder="Pick file" value={file} onChange={setFile} />
        <Group justify="flex-end">
          <Button onClick={upload} disabled={!file}>Upload</Button>
        </Group>
      </Stack>
    </Paper>
  );
}
