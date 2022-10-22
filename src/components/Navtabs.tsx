import { Tab } from "@headlessui/react"
import Link from "next/link";
import { linkIndexMap, navLinks, useGlobalContext } from "@/context/state";
import { useEffect, useState } from "react";

export const Navtabs = (props) => {
  const {currentPage} = useGlobalContext();
  const [selectedIndex, setSelectedIndex] = useState(linkIndexMap[currentPage]);
  useEffect(() => {
    setSelectedIndex(navLinks.map((link) => {
      return link.name
    }).indexOf(currentPage))

    console.log(currentPage, selectedIndex)
  }, [currentPage, selectedIndex, setSelectedIndex])

  return (
    <Tab.Group className="flex flex-col justify-evenly sm:flex-row" selectedIndex={selectedIndex}>
      <Tab.List>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          <Link href='/'>Dashboard</Link>
        </Tab>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          <Link href='/apps'>Apps</Link>
        </Tab>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          <Link href='/saves'>Saves</Link>
        </Tab>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          <Link href='/settings'>Settings</Link>
        </Tab>
      </Tab.List>
    </Tab.Group>
  )
}
