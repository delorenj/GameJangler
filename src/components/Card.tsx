import React from "react"

interface CardProps {
  url: string
  title: string
  description: string
}

export const Card: React.FC<CardProps> = (props: CardProps) => {
  const { url, title, description } = props
  return (
    <a
      href={url}
      target="_blank"
      rel="noopener noreferrer"
      className="flex h-40 w-64 flex-wrap rounded border-2 border-slate-700 p-4 drop-shadow-md -backdrop-hue-rotate-15 hover:drop-shadow-xl"
    >
      <h2 className="mb-4 text-2xl">{title}</h2>
      <p className="m-0 text-xl">{description}</p>
    </a>
  )
}
