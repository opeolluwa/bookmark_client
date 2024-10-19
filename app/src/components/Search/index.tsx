"use client";

import { MagnifyingGlassIcon } from "@heroicons/react/24/outline";
import { useState } from "react";

/**
 *  types definition for the search filed
 * takes the keyword TODO and the function to execute which will be imported from Rust core
 */
interface Props {
  onSearch: (city: string) => void;
  placeholder: string;
  className?: string
}

export default function SearchBar({  placeholder , className}: Props) {
  const [city, setCity] = useState("");

  // function handleSubmit(e: { preventDefault: () => void }) {
  //   e.preventDefault();
  //   onSearch(city);
  // }

  return (
    <form
      // onSubmit={handleSubmit}
      style={{
        position: "relative",
      }}
      className={className}
    >
      
      <label htmlFor="search box" className="sr-only">
        {" "}
        search 
      </label>
      
      <input
        className={" px-12 py-3  block w-full border-gray-400 dark:border-transparent bg-slate-50 focus:border-app-400 outline-none  border-none shadow placeholder:text-sm rounded-full " + className}
        type="text"
        placeholder={placeholder || "search"}
        value={city}
        onChange={(e) => setCity(e.target.value)}
      />
      <button
        type="submit"
        className="pr-12"
        style={{
          position: "absolute",
          left: "1.5rem",
          right:".5rem",
          top: "50%",
          display:"inline-block",
          transform: "translateY(-50%)",
          
        }}
      >
        <span className="sr-only">search</span>
        <MagnifyingGlassIcon className="w-5 h-5 font-semibold mr-8 text-gray-400" />
      </button>
    </form>
  );
}
