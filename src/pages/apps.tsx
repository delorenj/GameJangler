import { invoke } from "@tauri-apps/api/tauri"
import type { NextPage } from "next"
import Head from "next/head"
import Image from "next/image"
import { useEffect, useState } from "react";

import { Navbar } from "@/components/Navbar"
import { useGlobalContext } from "@/context/state";
import { Container } from "@/components/Container";

const Apps: NextPage = () => {
  const {currentPage, setCurrentPage} = useGlobalContext();

  useEffect(() => {
    setCurrentPage('Apps')
  }, [setCurrentPage]);

  return (
    <Container>
      <h1 className="m-0 self-center text-center text-6xl">Apps</h1>
    </Container>
  )
}

export default Apps
