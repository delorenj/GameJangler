import { invoke } from "@tauri-apps/api/tauri"
import type { NextPage } from "next"
import Head from "next/head"
import Image from "next/image"
import { useEffect, useState } from "react";

import { Navbar } from "@/components/Navbar"
import { useGlobalContext } from "@/context/state";

const Apps: NextPage = () => {
  const {currentPage, setCurrentPage} = useGlobalContext();

  useEffect(() => {
    setCurrentPage('Apps')
  }, []);

  return (
    <div className="flex min-h-screen flex-col bg-white">
      <Head>
        <title>Game Jangler</title>
        <meta name="description" content="Gaming cloud save manager" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="flex flex-col justify-center">
        <Navbar />
        <div className="flex h-[800px] justify-center">
          <h1 className="m-0 self-center text-center text-6xl">
            {currentPage}
          </h1>
        </div>
      </main>

      <footer className="flex flex-1 flex-grow-0 items-center justify-center border-t border-gray-200 py-4">
        <div>
          <a
            href="https://tauri.app/"
            target="_blank"
            rel="noopener noreferrer"
            className="flex flex-grow items-center justify-center p-4"
          >
            Powered by{" "}
            <span className="ml-2 h-6">
              <Image
                src="/tauri_logo_light.svg"
                alt="Vercel Logo"
                height={24}
                width={78}
              />
            </span>
          </a>
        </div>
      </footer>
    </div>
  )
}

export default Apps
