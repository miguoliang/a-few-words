import React from 'react';
import { userManager } from "../oidc";

const LoginPage: React.FC = () => {
  return (
    <div className="flex items-center justify-center h-screen bg-base-200">
      <div className="text-center">
        <h1 className="text-3xl font-bold mb-4">Welcome to Word Browser</h1>
        <button
          className="btn btn-primary"
          onClick={() => userManager.signinRedirect()}
        >
          Login
        </button>
      </div>
    </div>
  );
};

export default LoginPage;
