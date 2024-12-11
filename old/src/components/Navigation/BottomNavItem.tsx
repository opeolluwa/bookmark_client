"use client";
import View from "@/components/View";
import {
  BellIcon,
  Cog6ToothIcon,
  HomeIcon,
  UserIcon,
} from "@heroicons/react/24/outline";
import {
  BellIcon as SolidBellIcon,
  Cog6ToothIcon as SolidCog6ToothIcon,
  HomeIcon as SolidHomeIcon,
  UserIcon as SolidUserIcon,
} from "@heroicons/react/24/solid";
import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import React, { ReactNode, useState } from "react";

export interface ApplicationRoute {
  label: string;
  path?: string;
  icon?: ReactNode;
  alternateIcon?: ReactNode;
  action?: () => void;
}
const bottomNavigation: ApplicationRoute[] = [
  {
    label: "home",
    path: "/",
    icon: <HomeIcon />,
    alternateIcon: <SolidHomeIcon />,
  },
  {
    icon: <BellIcon />,
    alternateIcon: <SolidBellIcon />,
    label: "notification",
    path: "notification",
  },
  {
    icon: <UserIcon />,
    alternateIcon: <SolidUserIcon />,
    label: "profile",
    path: "profile",
  },

  {
    icon: <Cog6ToothIcon />,
    alternateIcon: <SolidCog6ToothIcon />,
    label: "settings",
    path: "settings",
  },
];

export default function BottomNavItem({
  icon,
  alternateIcon,
  label,
  path: slug,
}: ApplicationRoute) {
  const currentPath = usePathname();
  const previewClass =
    "flex flex-col justify-center items-center hover:text-app";
  const activeClass = "flex flex-col justify-center items-center text-app";
  const path = `/mobile/dashboard/${slug}`;
  const defaultPath = "/mobile/dashboard/";
  return (
    <div>
      <Link href={path} key={label}>
        <button className={currentPath == path ? activeClass : previewClass}>
          <span className="size-5">
            {currentPath == path ? alternateIcon : icon}
          </span>
          <span className="btm-nav-label text-[0.75rem] font-medium capitalize">
            {label}
          </span>
        </button>
      </Link>
    </div>
  );
}
