import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { MESSAGE_NAME, userManager } from "./oidc";
import Login from "./Login.tsx";

// check if this app is launched by a redirect uri of oidc provider
const url = new URL(window.location.href);
const code = url.searchParams.get("code");
const state = url.searchParams.get("state");
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
  });
} else if (code) {
  console.debug("code is present but state is missing");
  userManager.signoutRedirectCallback().then(() => {
    window.history.replaceState({}, document.title, location.pathname);
  });
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