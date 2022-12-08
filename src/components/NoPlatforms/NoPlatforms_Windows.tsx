import { useEffect, useState } from "react"

import { useGlobalContext } from "@/context/state"
import { ScanRequest, useScanner } from "@/hooks/scanner"
import { Platform } from "@/hooks/settings"

export const NoPlatforms_Windows: React.FC = () => {
  const { scanForPlatforms, response } = useScanner()
  const { serverMessages } = useGlobalContext()

  const startScan = async () => {
    const scanRequest: ScanRequest = {
      platformSet: {
        platforms: [Platform.STEAM],
      },
      rootPaths: ["/"],
    }
    await scanForPlatforms(scanRequest).then(() => {
      console.log("Done scanning")
    })
  }

  useEffect(() => {
    if (!response) return
    console.log(`Platforms Detected: ${JSON.stringify(response)}`)
  }, [response])

  useEffect(() => {
    if (!serverMessages) return
    console.log(`Message Detected: ${JSON.stringify(serverMessages)}`)
  }, [serverMessages])

  return (
    <div className="mx-auto mt-6 w-full max-w-sm focus-within:border-blue-400 focus-within:ring focus-within:ring-blue-300 focus-within:ring-opacity-40 dark:border-gray-700 dark:focus-within:border-blue-300">
      <button
        type="button"
        className="m-1 h-10 transform rounded-md bg-blue-500 px-4 py-2 text-white transition-colors duration-300 hover:bg-blue-400 focus:bg-blue-400 focus:outline-none"
        onClick={async () => await startScan()}
      >
        Scan for platforms
      </button>
    </div>
  )
}
