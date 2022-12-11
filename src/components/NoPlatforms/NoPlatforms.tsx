import { FC, useEffect, useRef, useState } from "react"

import { Loader } from "@/components/Loader"
import { LogoFooter } from "@/components/LogoFooter"
import { NoPlatforms_Darwin } from "@/components/NoPlatforms/NoPlatforms_Darwin"
import { NoPlatforms_Linux } from "@/components/NoPlatforms/NoPlatforms_Linux"
import { NoPlatforms_Windows } from "@/components/NoPlatforms/NoPlatforms_Windows"
import { OS, useSettings } from "@/hooks/settings"
import { useGlobalContext } from "@/context/state"

const components = {
  loader: Loader,
  [OS.WINDOWS]: NoPlatforms_Windows,
  [OS.DARWIN]: NoPlatforms_Darwin,
  [OS.LINUX]: NoPlatforms_Linux,
}

export const NoPlatforms: React.FC = () => {
  const { settings } = useSettings()
  const [PlatformSpecificButton, setPlatformSpecificButton] = useState<FC>(() => Loader)
  const { serverMessage, currentPage } = useGlobalContext()

  useEffect(() => {
    if (!settings || !settings.os) return
    console.log(`Settings OS detected!: ${settings.os}`)
    setPlatformSpecificButton(() => components[settings.os])
  }, [settings])

  return (
    <div className="container mx-auto px-6 py-16 pt-28 text-center">
      <div className="mx-auto max-w-lg">
        <h1 className="text-3xl font-bold text-gray-800 dark:text-white md:text-4xl">
          You haven't added any platforms yet! {currentPage}
        </h1>

        <p className="mt-6 text-gray-500 dark:text-gray-300">
          We can do a quick scan for you to find all locally installed platforms. Or, if
          you know where they are, you can just add them manually.
        </p>

        <PlatformSpecificButton />
        <p>{JSON.stringify(serverMessage)}</p>
      </div>
      <LogoFooter />
    </div>
  )
}
