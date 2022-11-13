import { invoke } from "@tauri-apps/api/tauri"
import type { NextPage } from "next"
import Head from "next/head"
import Image from "next/image"
import { useEffect, useState } from "react"

import { AppCarousel } from "@/components/AppCarousel"
import { Container } from "@/components/Container"
import { Navbar } from "@/components/Navbar"
import { SaveCarousel } from "@/components/SaveCarousel"
import { StorageCarousel } from "@/components/StorageCarousel"
import { useGlobalContext } from "@/context/state"

const Home: NextPage = () => {
  const { currentPage, setCurrentPage } = useGlobalContext()

  useEffect(() => {
    setCurrentPage("Home")
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
