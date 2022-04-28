import React, { useState } from 'react';
import clsx from 'clsx';
import { useTodos } from '../../providers/TodoProvider';

interface TextInputProps {
  placeholder?: string;
  todoText?: string;
  newTodo?: boolean;
  editing?: boolean;
}

const TextInput: React.FC<TextInputProps> = ({ todoText, placeholder, editing, newTodo }) => {
  const [text, setText] = useState('');
  const { addTodo } = useTodos();

  const handleSubmit = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key !== 'Enter') return;
    const description = e.currentTarget.value.trim();
    await addTodo(description);
    setText('');
  };

  const onChange = ({ target }: React.ChangeEvent<HTMLInputElement>) => setText(target.value);

  return (
    <input
      className={clsx({
        edit: editing,
        'new-todo': newTodo
      })}
      type="text"
      placeholder={placeholder}
      autoFocus
      value={text}
      onChange={onChange}
      onKeyDown={handleSubmit}
    />
  );
};

export default TextInput;
