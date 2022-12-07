import type { NextPage } from "next"
import { useEffect } from "react"

import { Container } from "@/components/Container"
import { useGlobalContext } from "@/context/state"
import { useSettings } from "@/hooks/settings"
import { NoPlatforms } from "@/components/NoPlatforms/NoPlatforms";
import * as OS from "os";
import { NoPlatforms_Linux } from "@/components/NoPlatforms/NoPlatforms_Linux";
import { NoPlatforms_Darwin } from "@/components/NoPlatforms/NoPlatforms_Darwin";
import { NoPlatforms_Windows } from "@/components/NoPlatforms/NoPlatforms_Windows";

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
      {!settings || (!settings.platforms && <NoPlatforms />)}
    </Container>
  );
}

export default Apps
