import { invoke } from "@tauri-apps/api/tauri"
import type { NextPage } from "next"
import Head from "next/head"
import Image from "next/image"
import { useEffect, useState } from "react";

import { Navbar } from "@/components/Navbar"
import { useGlobalContext } from "@/context/state";
import { Container } from "@/components/Container";
import { StorageCarousel } from "@/components/StorageCarousel";
import { AppCarousel } from "@/components/AppCarousel";
import { SaveCarousel } from "@/components/SaveCarousel";

const Home: NextPage = () => {
  const {currentPage, setCurrentPage} = useGlobalContext();

  useEffect(() => {
    setCurrentPage('Home')
  }, [setCurrentPage]);

  return (
    <Container>
      <div className='flex w-full'>
        <ul className='flex flex-1 flex-col w-1/2 gap-20'>
          <li><SaveCarousel /></li>
          <li><AppCarousel /></li>
          <li><StorageCarousel /></li>
        </ul>
      </div>
    </Container>
  )
}

export default Home
