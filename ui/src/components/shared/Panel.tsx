// src/components/shared/Panel.tsx
import React, { ReactNode } from 'react';

interface PanelProps {
  title: string;
  children: ReactNode;
  onSplit?: () => void;
  onStack?: () => void;
  onClose?: () => void;
}

const Panel: React.FC<PanelProps> = ({ title, children, onSplit, onStack, onClose }) => {
  return (
    <div style={{
      background: '#2a2a2a',
      border: '1px solid #333',
      borderRadius: '4px',
      height: '100%',
      display: 'flex',
      flexDirection: 'column'
    }}>
      <div style={{
        padding: '8px',
        borderBottom: '1px solid #333',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
        cursor: 'move' // Make header draggable
      }} className="panel-header">
        <span>{title}</span>
        <div style={{ display: 'flex', gap: '4px' }}>
          {onSplit && (
            <button onClick={onSplit} style={{ 
              padding: '2px 6px',
              background: '#444',
              border: 'none',
              borderRadius: '2px',
              color: '#fff',
              cursor: 'pointer'
            }}>
              Split
            </button>
          )}
          {onStack && (
            <button onClick={onStack} style={{ 
              padding: '2px 6px',
              background: '#444',
              border: 'none',
              borderRadius: '2px',
              color: '#fff',
              cursor: 'pointer'
            }}>
              Stack
            </button>
          )}
          {onClose && (
            <button onClick={onClose} style={{ 
              padding: '2px 6px',
              background: '#444',
              border: 'none',
              borderRadius: '2px',
              color: '#fff',
              cursor: 'pointer'
            }}>
              ×
            </button>
          )}
        </div>
      </div>
      <div style={{ 
        padding: '8px',
        flexGrow: 1,
        overflow: 'auto'
      }}>
        {children}
      </div>
    </div>
  );
};

export default Panel;


//import React from 'react';
//type ReactNode = React.ReactNode;
//
//interface PanelProps {
//  title: string;
//  children: ReactNode;
//}
//
//const Panel: React.FC<PanelProps> = ({ title, children }) => {
//  return (
//    <div style={{
//      background: '#ffff00',
//      border: '1px solid #333',
//      borderRadius: '4px',
//      height: '100%'
//    }}>
//      <div style={{
//        padding: '8px',
//        borderBottom: '1px solid #333',
//        display: 'flex',
//        justifyContent: 'space-between',
//        alignItems: 'center'
//      }}>
//        <span>{title}</span>
//        <div style={{ display: 'flex', gap: '4px' }}>
//          <button style={{ 
//            width: '12px', 
//            height: '12px', 
//            borderRadius: '50%', 
//            background: '#666' 
//          }}/>
//          <button style={{ 
//            width: '12px', 
//            height: '12px', 
//            borderRadius: '50%', 
//            background: '#666' 
//          }}/>
//        </div>
//      </div>
//      <div style={{ padding: '8px' }}>
//        {children}
//      </div>
//    </div>
//  );
//};
//
//export default Panel;