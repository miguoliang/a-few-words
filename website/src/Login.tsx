import { userManager } from "./oidc";

const Login = () => {
  return (
    <button
      onClick={() => {
        userManager.signinRedirect();
      }}
    >
      Login
    </button>
  );
};

export default Login;
