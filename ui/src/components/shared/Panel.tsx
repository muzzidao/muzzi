import React, { ReactNode } from 'react';

interface PanelProps {
  title: string;
  children: ReactNode;
}

const Panel: React.FC<PanelProps> = ({ title, children }) => {
  return (
    <div style={{
      background: '#2a2a2a',
      border: '1px solid #333',
      borderRadius: '4px',
      height: '100%'
    }}>
      <div style={{
        padding: '8px',
        borderBottom: '1px solid #333',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center'
      }}>
        <span>{title}</span>
        <div style={{ display: 'flex', gap: '4px' }}>
          <button style={{ 
            width: '12px', 
            height: '12px', 
            borderRadius: '50%', 
            background: '#666' 
          }}/>
          <button style={{ 
            width: '12px', 
            height: '12px', 
            borderRadius: '50%', 
            background: '#666' 
          }}/>
        </div>
      </div>
      <div style={{ padding: '8px' }}>
        {children}
      </div>
    </div>
  );
};

export default Panel;