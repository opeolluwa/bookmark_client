"use client";

import Link from "next/link";
import React, { useState } from "react";
import { useRouter } from "next/navigation";
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
        className="flex items-left justify-start lg:items-start my-2 rounded  ease-in-out  hover:text-app-600 py-2 text-gray-500 cursor-pointer"
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
