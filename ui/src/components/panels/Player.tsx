// src/components/panels/Player.tsx
import React, { useState } from 'react';
import Panel from '../shared/Panel';
import { PanelControlProps } from '../../types/panel';



const Player: React.FC<PanelControlProps> = ({ onSplit, onStack, onClose }) => {
  const [isPlaying, setIsPlaying] = useState(false);

  return (
    <Panel title="Player" onSplit={onSplit} onStack={onStack} onClose={onClose}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <div style={{ 
          height: '40px', 
          background: '#333', 
          borderRadius: '4px',
          padding: '8px'
        }}>
          Now Playing: No track selected
        </div>
        <div style={{ display: 'flex', gap: '8px', justifyContent: 'center' }}>
          <button onClick={() => {}}>◀◀</button>
          <button onClick={() => setIsPlaying(!isPlaying)}>
            {isPlaying ? '⏸️' : '▶️'}
          </button>
          <button onClick={() => {}}>▶▶</button>
        </div>
        <div style={{ 
          height: '4px', 
          background: '#444',
          borderRadius: '2px'
        }}>
          <div style={{ 
            width: '30%', 
            height: '100%', 
            background: '#666',
            borderRadius: '2px'
          }}/>
        </div>
      </div>
    </Panel>
  );
};
//const Player: React.FC<PanelControlProps> = ({ onSplit, onStack, onClose }) => {
//  const [isPlaying, setIsPlaying] = useState(false);
//
//  return (
//    <Panel title="Player">
//      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
//        <div style={{ 
//          height: '40px', 
//          background: '#333', 
//          borderRadius: '4px',
//          padding: '8px'
//        }}>
//          Now Playing: No track selected
//        </div>
//        <div style={{ display: 'flex', gap: '8px', justifyContent: 'center' }}>
//          <button onClick={() => {}}>◀◀</button>
//          <button onClick={() => setIsPlaying(!isPlaying)}>
//            {isPlaying ? '⏸️' : '▶️'}
//          </button>
//          <button onClick={() => {}}>▶▶</button>
//        </div>
//        <div style={{ 
//          height: '4px', 
//          background: '#444',
//          borderRadius: '2px'
//        }}>
//          <div style={{ 
//            width: '30%', 
//            height: '100%', 
//            background: '#666',
//            borderRadius: '2px'
//          }}/>
//        </div>
//      </div>
//    </Panel>
//  );
//};

export default Player;