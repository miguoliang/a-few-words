import React, { useEffect, useState } from 'react';
import { useRoutes, Navigate, BrowserRouter } from 'react-router-dom';
import { userManager } from "./oidc";
import { User } from "oidc-client-ts";
import getRoutes from './routes';

const AppRoutes: React.FC<{ isAuthenticated: boolean }> = ({ isAuthenticated }) => {
  const routes = getRoutes(isAuthenticated);
  const element = useRoutes([
    ...routes,
    { path: '*', element: <Navigate to="/" replace /> }
  ]);
  return element;
};

const App: React.FC = () => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const updateUser = async () => {
      try {
        const currentUser = await userManager.getUser();
        setUser(currentUser);
      } catch (error) {
        console.error("Error fetching user:", error);
        setUser(null);
      } finally {
        setLoading(false);
      }
    };

    updateUser();

    const handleUserLoaded = (user: User) => setUser(user);
    const handleUserUnloaded = () => setUser(null);

    userManager.events.addUserLoaded(handleUserLoaded);
    userManager.events.addUserUnloaded(handleUserUnloaded);

    return () => {
      userManager.events.removeUserLoaded(handleUserLoaded);
      userManager.events.removeUserUnloaded(handleUserUnloaded);
    };
  }, []);

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <BrowserRouter>
      <AppRoutes isAuthenticated={!!user} />
    </BrowserRouter>
  );
};

export default App;
