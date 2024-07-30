import { Provider } from "react-redux";
import { PersistGate } from "@plasmohq/redux-persist/integration/react";
import { launchWebAuthFlow } from "~content";
import { persistor, store, useAppDispatch, useAppSelector } from "~store";
import "~style.css";
import { useEffect } from "react";
import { setLogout } from "~auth-slice";
import { setWords } from "~words-slice";

const SidePanel = () => {
  return (
    <Provider store={store}>
      <PersistGate loading={null} persistor={persistor}>
        <AuthenticatedView />
      </PersistGate>
    </Provider>
  );
};

const AuthenticatedView = () => {
  const accessToken = useAppSelector((state) => state.auth.access_token);
  const dispatch = useAppDispatch();

  useEffect(() => {
    if (!accessToken) return;

    const fetchWords = async () => {
      try {
        const response = await fetch("http://localhost:8000/words", {
          headers: {
            Authorization: `Bearer ${store.getState().auth.access_token}`,
          },
        });

        if (response.ok) {
          const data = await response.json();
          dispatch(setWords(data));
        } else if (response.status === 401) {
          dispatch(setLogout());
        } else {
          console.error(`Error: ${response.statusText} (${response.status})`);
        }
      } catch (error) {
        console.error(`Fetch error: ${error.message}`);
      }
    };

    fetchWords();
  }, [accessToken, dispatch]);

  return (
    <div className="flex flex-col">
      {!accessToken && (
        <button
          className="m-5 bg-black text-white"
          onClick={() => launchWebAuthFlow()}
        >
          Click to authenticate
        </button>
      )}
      <WordList />
    </div>
  );
};

const WordList = () => {
  const words = useAppSelector((state) => state.words.words);
  console.log("Words in state:", words); // Debug log

  return (
    <div className="flex flex-col">
      {words?.length === 0 && <div>No words saved yet</div>}
      {words?.map((word, index) => (
        <WordCell key={index} word={word.word} />
      ))}
    </div>
  );
};

interface WordProps {
  word: string;
}

const WordCell = ({ word }: WordProps) => {
  return <div>{word}</div>;
};

export default SidePanel;
