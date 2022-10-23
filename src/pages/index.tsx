import { invoke } from "@tauri-apps/api/tauri"
import type { NextPage } from "next"
import Head from "next/head"
import Image from "next/image"
import { useEffect, useState } from "react";

import { Navbar } from "@/components/Navbar"
import { useGlobalContext } from "@/context/state";
import { Container } from "@/components/Container";

const Home: NextPage = () => {
  const {currentPage, setCurrentPage} = useGlobalContext();

  useEffect(() => {
    setCurrentPage('Home')
  }, [setCurrentPage]);

  return (
    <Container>
      <h1 className="m-0 self-center text-center text-6xl">
        Welcome to{" "}
        <a
          href="https://nextjs.org"
          target="_blank"
          rel="noopener noreferrer"
          className="text-blue-600 hover:underline focus:underline active:underline"
        >
          Game Jangler
        </a>
      </h1>
    </Container>
  )
}

export default Home
