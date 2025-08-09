import { create } from 'zustand';

interface SessionState {
  sessionId?: string;
  roles: string[];
  setSession: (id: string) => void;
  setRoles: (r: string[]) => void;
}

export const useSession = create<SessionState>((set) => ({
  roles: [],
  setSession: (sessionId) => set({ sessionId }),
  setRoles: (roles) => set({ roles }),
}));
