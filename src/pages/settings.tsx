import type { NextPage } from "next"
import { useEffect } from "react"

import { Container } from "@/components/Container"
import { useGlobalContext } from "@/context/state"

const Home: NextPage = () => {
  const { setCurrentPage } = useGlobalContext()

  useEffect(() => {
    setCurrentPage("Settings")
  }, [setCurrentPage])

  return (
    <Container>
      <h1 className="m-0 self-center text-center text-6xl">Settings</h1>
    </Container>
  )
}

export default Home
