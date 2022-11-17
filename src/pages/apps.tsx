import type { NextPage } from "next"
import { useEffect } from "react"

import { Container } from "@/components/Container"
import { useGlobalContext } from "@/context/state"

const Apps: NextPage = () => {
  const { setCurrentPage } = useGlobalContext()

  useEffect(() => {
    setCurrentPage("Apps")
  }, [setCurrentPage])

  return (
    <Container>
      <h1 className="m-0 self-center text-center text-6xl">Apps</h1>
    </Container>
  )
}

export default Apps
