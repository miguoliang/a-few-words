import React from 'react';
import { userManager } from "../oidc";
import AnimatedBackground from './AnimatedBackground';
import Logo from './Logo';

const LoginPage: React.FC = () => {
  return (
    <div className="relative flex items-center justify-center h-screen overflow-hidden">
      <AnimatedBackground />
      <div className="z-10">
        <Logo />
        <p className="text-gray-300 mb-8 text-center mt-3">
          Explore the world of words with our amazing tool!
        </p>
        <button
          className="relative w-full py-3 px-6 text-white font-semibold rounded-lg bg-gradient-to-r from-indigo-600 to-purple-700 hover:from-indigo-700 hover:to-purple-800 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-opacity-50 transform hover:scale-105 transition-all duration-300 shadow-lg overflow-hidden group"
          onClick={() => userManager.signinRedirect()}
        >
          <span className="relative z-10">Login to Get Started</span>
          <div className="absolute top-0 -inset-full h-full w-1/2 z-5 block transform -skew-x-12 bg-gradient-to-r from-transparent to-white opacity-20 group-hover:animate-shine" />
        </button>
      </div>
    </div>
  );
};

export default LoginPage;
