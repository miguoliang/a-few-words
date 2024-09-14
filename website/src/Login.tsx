import { userManager } from "./oidc";

const Login = () => {
  return (
    <div className="flex flex-col items-stretch justify-center gap-8 px-8 -mt-16 h-[100vh]">
      <button
        onClick={() => {
          userManager.signinRedirect();
        }}
      >
        Login
      </button>
    </div>
  );
};

export default Login;
