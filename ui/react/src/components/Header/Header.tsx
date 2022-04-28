import React from 'react';
import TextInput from '../TextInput';
import { motion } from 'framer-motion';

const Header: React.FC = () => {
  return (
    <header className="header">
      <motion.h1
        initial={{ scale: 0 }}
        animate={{ scale: 1 }}
        transition={{
          duration: 1,
          ease: 'easeInOut',
          delay: 0.2
        }}
      >
        todos
      </motion.h1>
      <TextInput newTodo placeholder="What needs to be done?" />
    </header>
  );
};

export default Header;
