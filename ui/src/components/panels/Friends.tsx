// src/components/panels/Friends.tsx
import React from 'react';
import Panel from '../shared/Panel';

const Friends = () => {
  const mockFriends = [
    { name: 'Friend 1', status: 'Online' },
    { name: 'Friend 2', status: 'Offline' },
  ];

  return (
    <Panel title="Friends">
      <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
        {mockFriends.map((friend, index) => (
          <div
            key={index}
            style={{
              padding: '8px',
              background: '#333',
              borderRadius: '4px',
              display: 'flex',
              justifyContent: 'space-between'
            }}
          >
            <span>{friend.name}</span>
            <span style={{ 
              color: friend.status === 'Online' ? '#4CAF50' : '#666'
            }}>
              {friend.status}
            </span>
          </div>
        ))}
      </div>
    </Panel>
  );
};

export default Friends;