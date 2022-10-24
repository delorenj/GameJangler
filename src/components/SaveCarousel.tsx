import { Card } from "@/components/Card";
import { Carousel } from "@/components/Carousel";

export const SaveCarousel = () => {

  return (
    <Carousel title="Latest Saves">
      <Card url="#" title="Test 1" description="This is a sample description" />
      <Card url="#" title="Test 2" description="This is a sample description" />
    </Carousel>
  )
}
