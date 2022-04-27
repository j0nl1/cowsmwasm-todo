import React from 'react';
import TodoItem from '../TodoItem';
import { Reorder } from 'framer-motion';
import { Todo } from '../../interfaces/todo';

interface TodoListProps {
  todos: Todo[];
  setTodos: any;
}

const TodoList: React.FC<TodoListProps> = ({ todos, setTodos }) => (
  <>
    <Reorder.Group className="todo-list" axis="y" values={todos} onReorder={setTodos}>
      {todos.map((todo, index) => (
        <Reorder.Item key={todo.id} value={todo}>
          <TodoItem index={index} todo={todo} />
        </Reorder.Item>
      ))}
    </Reorder.Group>
  </>
);

export default TodoList;
