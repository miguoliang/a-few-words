import { useStorage } from "@plasmohq/storage/hook"

import "~style.css"

import type { CardList, Card as CardType } from "~content"

const SidePanel = () => {
  const [cards] = useStorage<CardList>("cards", [])
  return (
    <ul className="p-2 gap-2 flex flex-col">
      {cards.map((card, index) => (
        <li key={index}>
          <Card card={card} />
        </li>
      ))}
    </ul>
  )
}

const Card = ({ card }: { card: CardType }) => {
  return (
    <div className="card bg-neutral text-neutral-content w-96">
      <div className="card-body items-center text-center">
        <h2 className="card-title">{card.front}</h2>
        <p>{card.back}</p>
        <div className="card-actions justify-end">
          <button className="btn btn-primary">Easy</button>
          <button className="btn btn-ghost">Hard</button>
        </div>
      </div>
    </div>
  )
}

export default SidePanel
