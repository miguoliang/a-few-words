import { RouteObject } from 'react-router-dom';
import Dashboard from './Dashboard';
import WordBrowser from './components/WordBrowser';
import LoginPage from './components/LoginPage';
import Overview from './components/Overview';

const getRoutes = (isAuthenticated: boolean): RouteObject[] => {
  if (isAuthenticated) {
    return [
      {
        path: '/',
        element: <Dashboard />,
        children: [
          { path: '', element: <Overview /> },
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
