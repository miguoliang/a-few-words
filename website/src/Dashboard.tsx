import React, { useState } from "react";
import { Outlet } from "react-router-dom";

// Updated Header component with collapse button
// Updated Header component with collapse button on the left
const Header: React.FC<{
  isSidebarOpen: boolean;
  toggleSidebar: () => void;
}> = ({ isSidebarOpen, toggleSidebar }) => {
  return (
    <header className="bg-primary text-primary-content p-4 flex items-center">
      <button className="text-xl mr-4" onClick={toggleSidebar}>
        {isSidebarOpen ? "←" : "→"}
      </button>
      <h1 className="text-2xl font-bold">Dashboard Header</h1>
    </header>
  );
};

const Dashboard: React.FC = () => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);

  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar isOpen={isSidebarOpen} />
      {/* Main content (adjustable width) */}
      <div
        className={`${
          isSidebarOpen ? "w-4/5" : "w-full"
        } bg-base-100 transition-all duration-300 flex flex-col`}
      >
        <Header 
          isSidebarOpen={isSidebarOpen}
          toggleSidebar={() => setIsSidebarOpen(!isSidebarOpen)}
        />
        {/* Main content area */}
        <div className="p-4 flex-grow overflow-auto">
          <h2 className="text-2xl font-bold mb-4">Dashboard</h2>
          <Outlet />
        </div>
      </div>
    </div>
  );
};

const Sidebar: React.FC<{ isOpen: boolean }> = ({ isOpen }) => {
  return (
    <div
      className={`${
        isOpen ? "w-1/5" : "w-0"
      } bg-base-200 transition-all duration-300 overflow-hidden`}
    >
      <div className="p-4 w-full">
        <h2 className="text-xl font-bold mb-4">Sidebar</h2>
        {/* Add sidebar content here */}
      </div>
    </div>
  );
};

export default Dashboard;
