// src/components/panels/Downloader.tsx
import React from 'react';
import Panel from '../shared/Panel';

import { PanelControlProps } from '../../types/panel';

const Downloader: React.FC<PanelControlProps> = ({ onSplit, onStack, onClose }) => {
  const mockDownloads = [
    { name: 'Track 1.mp3', progress: 45 },
    { name: 'Track 2.mp3', progress: 78 },
  ];

  return (
    <Panel title="Downloads" onSplit={onSplit} onStack={onStack} onClose={onClose}>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        {mockDownloads.map((download, index) => (
          <div
            key={index}
            style={{
              padding: '8px',
              background: '#333',
              borderRadius: '4px'
            }}
          >
            <div style={{ marginBottom: '4px' }}>{download.name}</div>
            <div style={{ 
              height: '4px', 
              background: '#444',
              borderRadius: '2px'
            }}>
              <div style={{ 
                width: `${download.progress}%`, 
                height: '100%', 
                background: '#666',
                borderRadius: '2px'
              }}/>
            </div>
          </div>
        ))}
      </div>
    </Panel>
  );
};
//const Downloader = () => {
//  const mockDownloads = [
//    { name: 'Track 1.mp3', progress: 45 },
//    { name: 'Track 2.mp3', progress: 78 },
//  ];
//
//  return (
//    <Panel title="Downloads">
//      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
//        {mockDownloads.map((download, index) => (
//          <div
//            key={index}
//            style={{
//              padding: '8px',
//              background: '#333',
//              borderRadius: '4px'
//            }}
//          >
//            <div style={{ marginBottom: '4px' }}>{download.name}</div>
//            <div style={{ 
//              height: '4px', 
//              background: '#444',
//              borderRadius: '2px'
//            }}>
//              <div style={{ 
//                width: `${download.progress}%`, 
//                height: '100%', 
//                background: '#666',
//                borderRadius: '2px'
//              }}/>
//            </div>
//          </div>
//        ))}
//      </div>
//    </Panel>
//  );
//};

export default Downloader;