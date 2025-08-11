import { Button, Group, NavLink, Paper, ScrollArea, Select, Stack, TextInput, Title } from '@mantine/core';
import { useEffect, useMemo, useState } from 'react';
import { api } from '../api/client';

interface OrgUnit { id: number; name: string; parent_ids: number[] }

export default function OrgUnits() {
  const [units, setUnits] = useState<OrgUnit[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [id, setId] = useState('');
  const [name, setName] = useState('');
  const [parents, setParents] = useState('');

  const idToUnit = useMemo(() => {
    const m = new Map<number, OrgUnit>();
    for (const u of units) m.set(u.id, u);
    return m;
  }, [units]);

  const childrenMap = useMemo(() => {
    const m = new Map<number, number[]>();
    for (const u of units) {
      for (const p of (u.parent_ids || [])) {
        if (!m.has(p)) m.set(p, []);
        m.get(p)!.push(u.id);
      }
    }
    return m;
  }, [units]);

  const roots = useMemo(() => units.filter(u => !u.parent_ids || u.parent_ids.length === 0), [units]);

  const load = async () => {
    const { data } = await api.get<OrgUnit[]>('/org/units');
    setUnits(data);
  };

  useEffect(() => { load(); }, []);

  const onSelect = (value: string | null) => {
    setSelectedId(value);
    if (value) {
      const uid = parseInt(value, 10);
      const u = idToUnit.get(uid);
      if (u) {
        setId(String(u.id));
        setName(u.name);
        setParents((u.parent_ids || []).join(','));
      }
    }
  };

  const upsert = async () => {
    const body = {
      id: parseInt(id, 10),
      name,
      parents: parents.trim() ? parents.split(',').map((s) => parseInt(s.trim(), 10)).filter((n) => !Number.isNaN(n)) : [],
    };
    await api.post('/org/units/upsert', body);
    await load();
  };

  const renderNode = (uid: number, depth = 0): JSX.Element => {
    const u = idToUnit.get(uid);
    if (!u) return <></>;
    const childIds = childrenMap.get(uid) || [];
    return (
      <NavLink key={uid} label={`${u.name} (id:${u.id})`} defaultOpened>
        {childIds.map((cid) => renderNode(cid, depth + 1))}
      </NavLink>
    );
  };

  return (
    <Paper p="md" radius="md" withBorder>
      <Stack>
        <Title order={4}>Org Units</Title>
        <Group grow>
          <Select label="Select unit" placeholder="Pick unit" value={selectedId} onChange={onSelect}
                  data={units.map(u => ({ value: String(u.id), label: `${u.name} (${u.id})` }))} searchable clearable />
          <TextInput label="ID" value={id} onChange={(e) => setId(e.currentTarget.value)} placeholder="e.g. 1" />
          <TextInput label="Name" value={name} onChange={(e) => setName(e.currentTarget.value)} placeholder="Manufacturing" />
          <TextInput label="Parents (comma-separated IDs)" value={parents} onChange={(e) => setParents(e.currentTarget.value)} placeholder="e.g. 1,2" />
          <Button onClick={upsert}>Save</Button>
        </Group>
        <ScrollArea h={400} offsetScrollbars>
          <Stack>
            {roots.map(r => renderNode(r.id))}
          </Stack>
        </ScrollArea>
      </Stack>
    </Paper>
  );
}
