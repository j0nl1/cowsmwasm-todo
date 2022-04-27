import React, { useEffect, useMemo, useState } from 'react';
import { motion } from 'framer-motion';
import Footer from '../Footer';
import TodoList from '../TodoList';
import { useTodos } from '../../providers/TodoProvider';
import { Todo, TodoStatus } from '../../interfaces/todo';

const filterTodos = (todos: Todo[], filter: any) => {
  if (filter === 'Open') return todos.filter((todo) => todo.status === TodoStatus.OPEN);
  if (filter === 'Completed') return todos.filter((todo) => todo.status === TodoStatus.COMPLETED);
  return todos;
};

const MainSection: React.FC = () => {
  const { getTodoList, account, filter } = useTodos();
  const [todos, setTodos] = useState<Todo[]>([]);
  const [showTodos, setShowTodos] = useState<boolean>(true);

  const filteredTodos = useMemo(() => filterTodos(todos, filter), [todos, filter]);

  useEffect(() => {
    const loadTodos = async () => setTodos(await getTodoList(account));
    loadTodos();
  }, []);

  return (
    <motion.section layout className="main">
      <span>
        <input className="toggle-all" type="checkbox" defaultChecked={false} />
        <label onClick={() => setShowTodos(!showTodos)} />
      </span>
      {showTodos && <TodoList todos={filteredTodos} setTodos={setTodos} />}
      <Footer />
    </motion.section>
  );
};

export default MainSection;
