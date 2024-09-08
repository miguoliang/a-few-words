import { userManager } from "./oidc";

const Login = () => {
  return (
    <div className="flex flex-col items-stretch justify-center gap-8 px-8 -mt-16 h-[100vh]">
      <div className="overflow-hidden h-[200px] w-[300px] rounded-3xl border-[5px] border-black border-solid mx-auto">
        <img src="public/banner.webp" alt="logo" className="mt-[-30px]" />
      </div>
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
