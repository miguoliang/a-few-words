import { RouteObject } from 'react-router-dom';
import Dashboard from './Dashboard';
import WordBrowser from './components/WordBrowser';
import LoginPage from './components/LoginPage';

const getRoutes = (isAuthenticated: boolean): RouteObject[] => {
  if (isAuthenticated) {
    return [
      {
        path: '/',
        element: <Dashboard />,
        children: [
          { index: true, element: <h2>Welcome to the Dashboard</h2> },
          { path: 'words', element: <WordBrowser /> },
          // Add other authenticated routes here
        ],
      },
    ];
  } else {
    return [
      { path: '/', element: <LoginPage /> },
    ];
  }
};

export default getRoutes;
