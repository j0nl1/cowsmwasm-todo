import React, { PropsWithChildren, useEffect, useState } from 'react';
import { loadKeplr, getSigner } from '../services/keplr';
import { TodoClient, createSignClient } from '../services/cosmwasm';
import Spinner from '../components/Spinner';

interface TodoContextValue extends TodoClient {
  filter: string;
  setFilter: React.Dispatch<React.SetStateAction<string>>;
}

export const TodoContext = React.createContext<TodoContextValue | null>(null);

const TodoProvider: React.FC<PropsWithChildren<{}>> = ({ children }) => {
  const [client, setClient] = useState<TodoClient | null>(null);
  const [filter, setFilter] = useState<string>('All');

  useEffect(() => {
    const loadClient = async () => {
      await loadKeplr('uni-2');
      const signer = await getSigner();
      const signingClient = await createSignClient(signer);
      setClient(signingClient);
    };
    loadClient();
  }, []);

  if (!client) return <Spinner />;

  return <TodoContext.Provider value={{ ...client, filter, setFilter }}>{children}</TodoContext.Provider>;
};

export const useTodos = () => {
  const context = React.useContext(TodoContext);
  if (!context) throw new Error('Provider is not instanced');
  return context;
};

export default TodoProvider;
