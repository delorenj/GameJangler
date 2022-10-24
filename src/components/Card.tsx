interface CardProps {
  url: string
  title: string
  description: string
}

export const Card: React.FC<CardProps> = ({ url, title, description }: CardProps) => (
  <a
    href={url}
    target="_blank"
    rel="noopener noreferrer"
    className="w-64 h-40 flex-wrap flex border-2 border-slate-700 rounded p-4 -backdrop-hue-rotate-15 drop-shadow-md hover:drop-shadow-xl"
  >
    <h2 className="mb-4 text-2xl">{title}</h2>
    <p className="m-0 text-xl">{description}</p>
  </a>
)
