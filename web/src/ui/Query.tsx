import { Button, Code, Group, Paper, Stack, TextInput, Title } from '@mantine/core';
import axios from 'axios';
import { useState } from 'react';
import { useSession } from '../store/session';

export default function Query() {
  const [ql, setQl] = useState('FIND SIMILAR "PPAP Level 3 submission warrant" IN semantic_en TOP 10');
  const [result, setResult] = useState('');
  const { sessionId } = useSession();

  const run = async () => {
    const { data } = await axios.post('http://localhost:8090/semanticql', { ql }, {
      headers: sessionId ? { 'X-Session-Id': sessionId } : undefined,
    });
    setResult(JSON.stringify(data, null, 2));
  };

  return (
    <Paper p="md" radius="md" withBorder>
      <Stack>
        <Title order={4}>SemanticQL</Title>
        <Group grow>
          <TextInput label="Query" value={ql} onChange={(e) => setQl(e.currentTarget.value)} />
          <Button onClick={run}>Run</Button>
        </Group>
        <Code block>{result}</Code>
      </Stack>
    </Paper>
  );
}
