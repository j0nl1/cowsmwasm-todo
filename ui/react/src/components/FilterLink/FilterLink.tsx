import React, { PropsWithChildren } from 'react';
import clsx from 'clsx';
import { useTodos } from '../../providers/TodoProvider';

const FilterLink: React.FC<PropsWithChildren<{}>> = ({ children }) => {
  const { filter, setFilter } = useTodos();

  return (
    <a
      href="#"
      type="button"
      className={clsx({ selected: children === filter })}
      style={{ cursor: 'pointer' }}
      onClick={() => setFilter(`${children}`)}
    >
      {children}
    </a>
  );
};

export default FilterLink;
