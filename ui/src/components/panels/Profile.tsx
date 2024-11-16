// src/components/panels/Profile.tsx
import React from 'react';
import Panel from '../shared/Panel';

const Profile = () => {
  return (
    <Panel title="Profile">
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <div style={{ 
          display: 'flex', 
          alignItems: 'center',
          gap: '8px'
        }}>
          <div style={{ 
            width: '40px', 
            height: '40px', 
            background: '#444',
            borderRadius: '50%'
          }}/>
          <span>Username</span>
        </div>
        <div>Status: Online</div>
      </div>
    </Panel>
  );
};

export default Profile;