import React, { useState } from "react";
import { Outlet, useNavigate, Link, useLocation } from "react-router-dom";
import { userManager } from "./oidc";
import { AnimatePresence } from 'framer-motion';
import PageTransition from './components/PageTransition';

// Updated Header component with collapse button
// Updated Header component with collapse button on the left
const Header: React.FC<{
  isSidebarOpen: boolean;
  toggleSidebar: () => void;
}> = ({ isSidebarOpen, toggleSidebar }) => {
  const navigate = useNavigate();
  const handleLogout = () => {
    // Sign out the user
    userManager.signoutRedirect();
    // Redirect to login page
    navigate('/login');
  };

  return (
    <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4 flex items-center justify-between shadow-sm">
      {/* Left side: Sidebar Toggle & Title */}
      <div className="flex items-center gap-4">
        {/* Sidebar Toggle Button */}
        <button
          className="btn btn-ghost btn-circle hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          onClick={toggleSidebar}
          aria-label="Toggle Sidebar"
        >
          {isSidebarOpen ? (
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6 text-gray-600 dark:text-gray-300">
              <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          ) : (
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6 text-gray-600 dark:text-gray-300">
              <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
          )}
        </button>
        <h1 className="text-xl font-semibold text-gray-800 dark:text-gray-100">
          Learning Hub
        </h1>
      </div>

      {/* Right side: Help, Theme Toggle & User Menu */}
      <div className="flex items-center gap-3">
        {/* Help Button */}
        <button
          className="btn btn-ghost btn-circle hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          aria-label="Help"
        >
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6 text-gray-600 dark:text-gray-300">
            <path strokeLinecap="round" strokeLinejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
          </svg>
        </button>

        {/* Dark Mode Toggle Button */}
        <button
          onClick={() => document.documentElement.classList.toggle('dark')}
          className="btn btn-ghost btn-circle hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          aria-label="Toggle Dark Mode"
        >
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6 text-gray-600 dark:text-gray-300">
            <path strokeLinecap="round" strokeLinejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z" />
          </svg>
        </button>

        {/* User Menu Dropdown */}
        <div className="dropdown dropdown-end">
          <label
            tabIndex={0}
            className="btn btn-ghost btn-circle hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            aria-label="User Menu"
          >
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6 text-gray-600 dark:text-gray-300">
              <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" />
            </svg>
          </label>
          <ul tabIndex={0} className="mt-3 z-[1] p-2 shadow-lg menu menu-sm dropdown-content bg-white dark:bg-gray-800 rounded-lg w-52 border border-gray-100 dark:border-gray-700">
            <li><a className="hover:bg-gray-50 dark:hover:bg-gray-700 dark:text-gray-100">Profile</a></li>
            <li><a className="hover:bg-gray-50 dark:hover:bg-gray-700 dark:text-gray-100">Settings</a></li>
            <li><a onClick={handleLogout} className="hover:bg-gray-50 dark:hover:bg-gray-700 dark:text-gray-100">Logout</a></li>
          </ul>
        </div>
      </div>
    </header>
  );
};

const Dashboard: React.FC = () => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const location = useLocation();

  return (
    <div className="flex h-screen overflow-hidden bg-gray-50 dark:bg-gray-900">
      <Sidebar isOpen={isSidebarOpen} />
      <div
        className={`${
          isSidebarOpen ? "w-[90%]" : "w-full"
        } bg-gray-50 dark:bg-gray-900 transition-all duration-300 flex flex-col`}
      >
        <Header
          isSidebarOpen={isSidebarOpen}
          toggleSidebar={() => setIsSidebarOpen(!isSidebarOpen)}
        />
        <div className="p-6 flex-grow overflow-auto">
          <div className="max-w-9xl mx-auto">
            <AnimatePresence mode="wait">
              <PageTransition key={location.pathname}>
                <Outlet />
              </PageTransition>
            </AnimatePresence>
          </div>
        </div>
      </div>
    </div>
  );
};

const Sidebar: React.FC<{ isOpen: boolean }> = ({ isOpen }) => {
  return (
    <div
      className={`${isOpen ? "w-[10%]" : "w-0"
        } bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 transition-all duration-300 overflow-hidden shadow-sm`}
    >
      <div className="p-6 w-full">
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-6">Navigation</h2>
        <nav className="space-y-2">
          {/* Overview Link */}
          <Link to="/" className="flex items-center gap-2 text-gray-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700 rounded-lg p-2 transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-5 h-5">
              <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
            </svg>
            Overview
          </Link>

          {/* Word Browser Link */}
          <Link to="/words" className="flex items-center gap-2 text-gray-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700 rounded-lg p-2 transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-5 h-5">
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25" />
            </svg>
            Words
          </Link>
        </nav>
      </div>
    </div>
  );
};

export default Dashboard;
