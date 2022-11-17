import React from "react"

import { Card } from "@/components/Card"
import { Carousel } from "@/components/Carousel"

export const AppCarousel: React.FC = () => {
  return (
    <Carousel title="My Apps">
      <Card url="#" title="Test 1" description="This is a sample description" />
      <Card url="#" title="Test 2" description="This is a sample description" />
      <Card url="#" title="Test 3" description="This is a sample description" />
    </Carousel>
  )
}
