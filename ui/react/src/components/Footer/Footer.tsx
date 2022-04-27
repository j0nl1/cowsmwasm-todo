import React from 'react';
import FilterLink from '../FilterLink';

const FILTERS = ['All', 'Open', 'Completed'];

const Footer: React.FC = () => (
  <footer className="footer">
    <ul className="filters">
      {FILTERS.map((filter) => (
        <li key={filter}>
          <FilterLink>{filter}</FilterLink>
        </li>
      ))}
    </ul>
  </footer>
);

export default Footer;
