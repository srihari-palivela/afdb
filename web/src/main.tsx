import { createRoot } from 'react-dom/client';
import { MantineProvider, createTheme, MantineColorsTuple } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import '@fontsource-variable/inter/index.css';
import App from './ui/App';

const slate: MantineColorsTuple = [
  '#f6f7f9',
  '#e9ecf2',
  '#cfd6e2',
  '#b3bfd0',
  '#9eafc6',
  '#8fa5c0',
  '#889fbf',
  '#758bab',
  '#677c9a',
  '#5a6d88',
];

const theme = createTheme({
  fontFamily: 'InterVariable, Inter, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif',
  headings: { fontFamily: 'InterVariable, Inter, ui-sans-serif, system-ui' },
  colors: { slate },
  primaryColor: 'slate',
  defaultRadius: 'md',
});

createRoot(document.getElementById('root')!).render(
  <MantineProvider theme={theme} defaultColorScheme="dark">
    <Notifications position="top-right" />
    <App />
  </MantineProvider>
);
