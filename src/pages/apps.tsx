import type { NextPage } from "next"
import { useEffect } from "react"

import { Container } from "@/components/Container"
import { NoPlatforms_Windows, NoPlatforms_Linux, NoPlatforms_Darwin } from "@/components/NoPlatforms"
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
      {settings && settings.platforms && (
        <h1 className="m-0 self-center text-center text-6xl">Apps</h1>
      )}
      {settings && !settings.platforms && settings.os === OS.DAWRIN && <NoPlatforms_Darwin />}
      {settings && !settings.platforms && settings.os === OS.WINDOWS && <NoPlatforms_Windows />}
      {settings && !settings.platforms && settings.os === OS.LINUX && <NoPlatforms_Linux />}

      {!settings && (
        <h4 className="m-0 self-center text-center text-4xl">Loading Settings...</h4>
      )}
    </Container>
  );
}

export default Apps
