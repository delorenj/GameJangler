import type { NextPage } from "next"
import { useEffect } from "react"

import { Container } from "@/components/Container"
import { NoPlatforms } from "@/components/NoPlatforms"
import { useGlobalContext } from "@/context/state"
import { useSettings } from "@/hooks/settings"

const Apps: NextPage = () => {
  const { settings } = useSettings()
  const { setCurrentPage } = useGlobalContext()

  useEffect(() => {
    setCurrentPage("Apps")
  }, [setCurrentPage])

  return (
    <Container>
      {settings?.platforms && (
        <h1 className="m-0 self-center text-center text-6xl">Apps</h1>
      )}
      {!settings?.platforms && <NoPlatforms />}
    </Container>
  )
}

export default Apps
