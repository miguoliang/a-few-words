import React from 'react';
import { userManager } from "../oidc";
import AnimatedBackground from './AnimatedBackground';

const LoginPage: React.FC = () => {
  return (
    <div className="relative flex items-center justify-center h-screen overflow-hidden">
      <AnimatedBackground />
      <div className="z-10 bg-gray-800 bg-opacity-80 p-8 rounded-lg shadow-2xl transform hover:scale-105 transition-transform duration-300">
        <h1 className="text-4xl font-bold mb-6 text-center bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-teal-300">
          Welcome to Word Browser
        </h1>
        <p className="text-gray-300 mb-8 text-center">
          Explore the world of words with our amazing tool!
        </p>
        <button
          className="w-full py-3 px-6 text-white font-semibold rounded-lg bg-gradient-to-r from-indigo-600 to-purple-700 hover:from-indigo-700 hover:to-purple-800 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-opacity-50 transform hover:scale-105 transition-all duration-300 shadow-lg"
          onClick={() => userManager.signinRedirect()}
        >
          Login to Get Started
        </button>
      </div>
    </div>
  );
};

export default LoginPage;
