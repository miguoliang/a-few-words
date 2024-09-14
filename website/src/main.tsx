import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./Logo.tsx";
import "./index.css";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { MESSAGE_NAME, userManager } from "./oidc";
import Login from "./Login.tsx";

// Check if this app is launched by a redirect uri of oidc provider
const url = new URL(window.location.href);
const code = url.searchParams.get("code");
const state = url.searchParams.get("state");

// Function to check user authorization
const checkUserAuthorization = async () => {
  try {
    const user = await userManager.getUser();
    if (user && !user.expired) {
      console.debug("User is authorized");
      // You can add additional logic here for authorized users
    } else {
      console.debug("User is not authorized");
      // You can add logic here for unauthorized users, e.g., redirect to login
    }
  } catch (error) {
    console.error("Error checking user authorization:", error);
  }
};

if (code && state) {
  userManager.signinRedirectCallback().then((user) => {
    window.history.replaceState({}, document.title, location.pathname);
    window.postMessage(
      {
        type: MESSAGE_NAME,
        id_token: user?.id_token,
        access_token: user?.access_token,
        refresh_token: user?.refresh_token,
        expires_at: user?.expires_at,
      },
      "*"
    );
    checkUserAuthorization(); // Check authorization after successful sign-in
  });
} else if (code) {
  console.debug("code is present but state is missing");
  userManager.signoutRedirectCallback().then(() => {
    window.history.replaceState({}, document.title, location.pathname);
    checkUserAuthorization(); // Check authorization after sign-out
  });
} else {
  // If no code or state, check authorization on app load
  checkUserAuthorization();
}

window.addEventListener("message", (event) => {
  if (event.source !== window) {
    return;
  }
  const message = event.data;
  if (message.type === "logout") {
    console.debug(
      "Received logout message from the content scripting",
      message
    );
    // Clear local tokens
    userManager.signoutRedirect({
      extraQueryParams: {
        client_id: import.meta.env.VITE_OIDC_CLIENT_ID,
        logout_uri: import.meta.env.VITE_OIDC_POST_LOGOUT_REDIRECT_URI,
        response_type: "code",
      },
    });
  }
});

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/login" element={<Login />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>
);
