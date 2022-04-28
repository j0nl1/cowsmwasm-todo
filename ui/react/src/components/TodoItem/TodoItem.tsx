import React from 'react';
import clsx from 'clsx';
import { motion } from 'framer-motion';
import { Todo, TodoStatus } from '../../interfaces/todo';
import { useTodos } from '../../providers/TodoProvider';

const variants = {
  hidden: {
    opacity: 0
  },
  visible: ({ delay }: { delay: number }) => ({
    opacity: 1,
    transition: {
      delay,
      duration: 1
    }
  })
};

interface TodoItemProps {
  index: number;
  todo: Todo;
}

const TodoItem: React.FC<TodoItemProps> = ({ index, todo }) => {
  const { changeTodoStatus, deleteTodo } = useTodos();

  const isCompleted = todo.status === TodoStatus.COMPLETED;

  return (
    <motion.div
      className={clsx({
        completed: isCompleted
      })}
      custom={{ delay: (index + 1) * 0.1 }}
      initial="hidden"
      animate="visible"
      exit="hidden"
      variants={variants}
      layoutId={todo.id.toString()}
    >
      <div className="view">
        <input className="toggle" type="checkbox" checked={isCompleted} onChange={() => changeTodoStatus(todo.id, TodoStatus.COMPLETED)} />
        <label>{todo.description}</label>
        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ cursor: 'pointer', scale: 1.5 }}
          type="button"
          className="destroy"
          onClick={() => deleteTodo(todo.id)}
        />
      </div>
    </motion.div>
  );
};

export default TodoItem;
