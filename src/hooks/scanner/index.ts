import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"

import { Platform, PlatformInstance } from "@/hooks/settings"

export interface ScanRequest {
  platforms: Platform[]
  rootPaths: string[]
}
export const useScanner = () => {
  const [scanningInProgress, setScanningInProgress] = useState<boolean>(false)
  const [response, setResponse] = useState<PlatformInstance[]>()

  const scanForPlatforms = async (scanRequest: ScanRequest) => {
    setScanningInProgress(true)
    invoke<PlatformInstance[]>("scan_for_platforms", {
      platforms: scanRequest.platforms,
      rootPaths: scanRequest.rootPaths,
    })
      .then((response) => {
        setResponse(response)
      })
      .finally(() => {
        setScanningInProgress(false)
      })
      .catch((err: string) => {
        setScanningInProgress(false)
        console.log(`There was an error scanning for platforms: ${err}`)
      })
  }

  return { scanningInProgress, scanForPlatforms, response }
}
