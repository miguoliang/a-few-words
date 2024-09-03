import { UserManager } from "oidc-client-ts";

const userManager = new UserManager({
  client_id: import.meta.env.VITE_OIDC_CLIENT_ID,
  redirect_uri: import.meta.env.VITE_OIDC_REDIRECT_URI,
  response_type: "code",
  scope: "openid profile email",
  authority: import.meta.env.VITE_OIDC_AUTHORITY,
  post_logout_redirect_uri: import.meta.env.VITE_OIDC_POST_LOGOUT_REDIRECT_URI,
});

export const MESSAGE_NAME = "a_few_words_oidc";

export { userManager };
