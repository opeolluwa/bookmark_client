"use client";

import Link from "next/link";
import { Route } from "./routes";

export default function NavigationTab({
  icon,
  path,
  name,
}: Route) {
  return (
    <div>
      <Link
        href={path}
        className="flex items-left justify-start lg:items-start my-1 rounded  ease-in-out hover:text-white  hover:bg-app-400 py-3 text-gray-500 cursor-pointer max-w-full px-5"
      >
        <span className="cursor-pointer">
          <span className="sr-only"> </span>
          <div className="gap-3 justify-left  flex items-center capitalize">
            {icon} <span className="">{name}</span>
          </div>
        </span>
      </Link>
    </div>
  );
}
