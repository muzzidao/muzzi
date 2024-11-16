// src/components/panels/Settings.tsx
import React from 'react';
import Panel from '../shared/Panel';

const Settings = () => {
  return (
    <Panel title="Settings">
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <div style={{
          padding: '8px',
          background: '#333',
          borderRadius: '4px',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center'
        }}>
          <span>Show Offline Friends</span>
          <input type="checkbox" />
        </div>
        <div style={{
          padding: '8px',
          background: '#333',
          borderRadius: '4px',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center'
        }}>
          <span>Enable Discovery</span>
          <input type="checkbox" />
        </div>
      </div>
    </Panel>
  );
};

export default Settings;