import { Button, FileInput, Group, Paper, Stack, TextInput, Title } from '@mantine/core';
import axios from 'axios';
import { useState } from 'react';

export default function Ingestion() {
  const [sourceApp, setSourceApp] = useState('confluence');
  const [orgHint, setOrgHint] = useState('Manufacturing/QA');
  const [file, setFile] = useState<File | null>(null);

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
    await axios.post('http://localhost:8090/ingest', body);
    alert('Ingested!');
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
