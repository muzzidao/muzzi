// src/components/panels/Discovery.tsx
import React from 'react';
import Panel from '../shared/Panel';
import { PanelControlProps } from '../../types/panel';

const Discovery: React.FC<PanelControlProps> = ({ onSplit, onStack, onClose }) => {
  return (
    <Panel title="Discovery" onSplit={onSplit} onStack={onStack} onClose={onClose}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        <input
          type="text"
          placeholder="Find users..."
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
          No users found
        </div>
      </div>
    </Panel>
  );
};
//const Discovery = () => {
//  return (
//    <Panel title="Discovery">
//      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
//        <input
//          type="text"
//          placeholder="Find users..."
//          style={{
//            padding: '8px',
//            background: '#333',
//            border: 'none',
//            borderRadius: '4px'
//          }}
//        />
//        <div style={{ 
//          minHeight: '100px', 
//          background: '#333',
//          borderRadius: '4px',
//          padding: '8px'
//        }}>
//          No users found
//        </div>
//      </div>
//    </Panel>
//  );
//};

export default Discovery;