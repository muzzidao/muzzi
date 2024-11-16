// src/components/panels/Playlist.tsx
import React from 'react';
import Panel from '../shared/Panel';

const Playlist = () => {
  const mockTracks = [
    'Track 1',
    'Track 2',
    'Track 3'
  ];

  return (
    <Panel title="Playlist">
      <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
        {mockTracks.map((track, index) => (
          <div
            key={index}
            style={{
              padding: '8px',
              background: '#333',
              borderRadius: '4px',
              cursor: 'pointer'
            }}
          >
            {track}
          </div>
        ))}
      </div>
    </Panel>
  );
};

export default Playlist;