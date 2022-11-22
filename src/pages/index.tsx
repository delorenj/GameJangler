import type { GetStaticProps, NextPage } from "next"
import Head from "next/head"
import Image from "next/image"
import { useEffect } from "react"

import { AppCarousel } from "@/components/AppCarousel"
import { Container } from "@/components/Container"
import { Navbar } from "@/components/Navbar"
import { SaveCarousel } from "@/components/SaveCarousel"
import { StorageCarousel } from "@/components/StorageCarousel"
import { ServerState, useGlobalContext } from "@/context/state"
import { invoke } from "@tauri-apps/api/tauri";


const Home: NextPage = () => {
  const { setCurrentPage } = useGlobalContext()
  useEffect(() => {
    setCurrentPage("Home")
    const fetch = async () => {
      return await invoke<string>("load_settings")
    }
    const result = fetch().catch(console.error)
  }, [setCurrentPage])

  return (
    <Container>
      <div className="flex w-full">
        <ul className="flex w-1/2 flex-1 flex-col gap-20">
          <li>
            <SaveCarousel />
          </li>
          <li>
            <AppCarousel />
          </li>
          <li>
            <StorageCarousel />
          </li>
        </ul>
      </div>
    </Container>
  )
}

export default Home
