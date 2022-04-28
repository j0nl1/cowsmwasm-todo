import React from 'react';
import Header from './components/Header';
import MainSection from './components/MainSection';
import TodoProvider from './providers/TodoProvider';

const App: React.FC = () => {
  return (
    <TodoProvider>
      <div>
        <Header />
        <MainSection />
      </div>
    </TodoProvider>
  );
};

export default App;
