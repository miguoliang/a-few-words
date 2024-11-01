import React from 'react';
import PageTransition from './PageTransition';

const Overview: React.FC = () => {
  return (
    <PageTransition>
      <div className="space-y-4">
        {/* ... existing content ... */}
        <h1>Overview</h1>
      </div>
    </PageTransition>
  );
};

export default Overview; 