import { useStorage } from "@plasmohq/storage/hook"

import "~style.css"

import type { SentenceList } from "~content"

const SidePanel = () => {
  const [cards, setCards] = useStorage<SentenceList>("cards", [])
  const removeCard = (index: number) => {
    const newCards = [...cards]
    newCards.splice(index, 1)
    setCards(newCards)
  }
  return (
    <ul className="p-2 gap-2 flex flex-col">
      {cards.map((card, index) => (
        <li key={index}>
          <div className="card bg-neutral text-neutral-content w-full">
            <div className="card-actions justify-end mr-2 mt-2">
              <button
                className="btn btn-square btn-sm"
                onClick={() => removeCard(index)}>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-6 w-6"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>
            <div className="card-body items-center text-center">
              <h2 className="card-title">{card.front}</h2>
            </div>
          </div>
        </li>
      ))}
    </ul>
  )
}

export default SidePanel
