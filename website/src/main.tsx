import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { UserManager } from "oidc-client-ts";

const userManager = new UserManager({
  client_id: import.meta.env.VITE_OIDC_CLIENT_ID,
  redirect_uri: import.meta.env.VITE_OIDC_REDIRECT_URI,
  response_type: "code",
  scope: "openid profile email",
  authority: import.meta.env.VITE_OIDC_AUTHORITY,
  post_logout_redirect_uri: import.meta.env.VITE_OIDC_POST_LOGOUT_REDIRECT_URI,
});

// check if this app is launched by a redirect uri of oidc provider
const url = new URL(window.location.href);
const code = url.searchParams.get("code");
const state = url.searchParams.get("state");
if (code && state) {
  userManager.signinRedirectCallback().then(() => {
    window.history.replaceState({}, document.title, location.pathname);
  });
} else if (code) {
  console.log("code is present but state is missing");
  userManager.signoutRedirectCallback().then(() => {
    window.history.replaceState({}, document.title, location.pathname);
  });
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
