import { Tab } from "@headlessui/react"

import { Navtabs } from "@/components/Navtabs"

export const Navbar = () => {
  return (
    <nav className="py-2.5">
      <div className="container flex flex-wrap items-center justify-between px-4 ">
        <a href="#" className="flex items-center">
          <img
            src="https://flowbite.com/docs/images/logo.svg"
            className="mr-3 h-6 sm:h-9"
            alt="Game Jangler Logo"
          />
          <span className="self-center whitespace-nowrap text-xl font-semibold text-blue-500">
            Game Jangler
          </span>
        </a>
        <button
          data-collapse-toggle="navbar-default"
          type="button"
          className="ml-3 inline-flex items-center rounded-lg p-2 text-sm text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600 md:hidden"
          aria-controls="navbar-default"
          aria-expanded="false"
        >
          <span className="sr-only">Open main menu</span>
          <svg
            className="h-6 w-6"
            aria-hidden="true"
            fill="currentColor"
            viewBox="0 0 20 20"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fill-rule="evenodd"
              d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
              clip-rule="evenodd"
            ></path>
          </svg>
        </button>
        <div className="hidden w-full md:block" id="navbar-default">
          <ul className="mx-auto mt-4 flex max-w-full flex-col bg-blue-100 p-4 md:mt-0 md:flex-row md:space-x-8 md:border-0 md:text-sm md:font-medium"></ul>
        </div>
      </div>
    </nav>
    // <Tab.Group>
    //   <Tab.List>
    //     <Tab>Dashboard</Tab>
    //     <Tab>Apps</Tab>
    //     <Tab>Saves</Tab>
    //     <Tab>Settings</Tab>
    //   </Tab.List>
    // </Tab.Group>
  )
}
