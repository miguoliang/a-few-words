import banner from "data-base64:~assets/banner.webp"
import { Navigate } from "react-router-dom"

import { isExpired, launchWebAuthFlow } from "~content"
import { useAppSelector } from "~store"

const WelcomeView = () => {
  const accessToken = useAppSelector((state) => state.auth.access_token)
  if (isExpired(accessToken)) {
    chrome.runtime.sendMessage({ type: "logout" })
  } else {
    return <Navigate to="/word-list" />
  }

  return (
    <div className="flex flex-col items-stretch justify-center gap-8 px-8 -mt-16 h-[100vh]">
      <div className="overflow-hidden h-[200px] rounded-3xl border-[5px] border-black border-solid">
        <img src={banner} alt="logo" className="mt-[-30px]" />
      </div>
      <button
        className="m-5 bg-black block text-white rounded-full text-xl p-3"
        onClick={async () => await launchWebAuthFlow()}>
        💪<span className="ml-5">Get Started</span>
      </button>
      <div className="grid grid-cols-2 gap-2">
        <button
          className="text-blue-500 underline"
          onClick={() =>
            chrome.runtime.sendMessage({
              action: "openUrl",
              url: "https://miguoliang.com/a-few-words/privacy-policy.html"
            })
          }>
          Privacy Policy
        </button>
        <button
          className="text-blue-500 underline"
          onClick={() =>
            chrome.runtime.sendMessage({
              action: "openUrl",
              url: "https://miguoliang.com/a-few-words/terms-and-conditions.html"
            })
          }>
          Terms and Conditions
        </button>
      </div>
    </div>
  )
}

export default WelcomeView
