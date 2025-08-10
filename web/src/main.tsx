import { createRoot } from 'react-dom/client';
import { MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import App from './ui/App';

createRoot(document.getElementById('root')!).render(
  <MantineProvider defaultColorScheme="dark">
    <Notifications />
    <App />
  </MantineProvider>
);
