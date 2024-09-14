import { useEffect, useState } from "react";
import { userManager } from "./oidc";
import { User } from "oidc-client-ts";

const LoginButton = () => {
  return <button
    onClick={() => {
      userManager.signinRedirect();
    }}
  >
    Login
  </button>
}

const LogoutButton = () => {
  return <button
    onClick={() => {
      const clientId = import.meta.env.VITE_OIDC_CLIENT_ID;
      const logoutUri = import.meta.env.VITE_OIDC_POST_LOGOUT_REDIRECT_URI;

      if (!clientId || !logoutUri) {
        console.error("Missing OIDC configuration");
        return;
      }

      userManager.signoutRedirect({
        extraQueryParams: {
          client_id: clientId,
          logout_uri: logoutUri,
          response_type: "code",
        },
      });
    }}
  >
    Logout
  </button>
}

export default function App() {
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    const updateUser = async () => {
      try {
        const currentUser = await userManager.getUser();
        setUser(currentUser);
      } catch (error) {
        console.error("Error fetching user:", error);
        setUser(null);
      }
    };

    updateUser();

    // Add event listeners for user changes
    const handleUserLoaded = (user: User) => setUser(user);
    const handleUserUnloaded = () => setUser(null);

    userManager.events.addUserLoaded(handleUserLoaded);
    userManager.events.addUserUnloaded(handleUserUnloaded);

    return () => {
      userManager.events.removeUserLoaded(handleUserLoaded);
      userManager.events.removeUserUnloaded(handleUserUnloaded);
    };
  }, []);

  return (
    <div className="flex flex-col items-stretch justify-center gap-8 px-8 -mt-16 h-[100vh]">
      {user ? <LogoutButton /> : <LoginButton />}
    </div>
  )
}