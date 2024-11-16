// src/components/panels/Search.tsx
import React, { useState } from 'react';
import Panel from '../shared/Panel';

const Search = () => {
  const [searchTerm, setSearchTerm] = useState('');

  return (
    <Panel title="Search">
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <input
          type="text"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          placeholder="Search tracks..."
          style={{
            padding: '8px',
            background: '#333',
            border: 'none',
            borderRadius: '4px'
          }}
        />
        <div style={{ 
          minHeight: '100px', 
          background: '#333',
          borderRadius: '4px',
          padding: '8px'
        }}>
          Search Results
        </div>
      </div>
    </Panel>
  );
};

export default Search;