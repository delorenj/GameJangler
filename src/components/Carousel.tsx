import { Card } from "@/components/Card";
interface CarouselProps {
  title: string
  children: React.FC<typeof Card>[]
}

export const Carousel: React.FC<CarouselProps> = ({title, children}) => {

  return (
    <>
      <h1 className='block text-2xl mb-4'>{title}</h1>
      <ul className='flex flex-1 flex-row gap-12'>
        {children.map(el => {
          return <li>{el}</li>
        })}
      </ul>
    </>
  )
}
