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
import { useRouter } from "next/navigation";
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
  const [currentIcon, setIcon] = useState<ReactNode>(icon);
  const router = useRouter();
  const previewClass =
    "flex flex-col justify-center items-center hover:text-app";
  const activeClass =
    "flex flex-col justify-center items-center text-app active";
  const path = `/mobile/dashboard/${slug}`;
  return (
    <div>
      <Link
        onBlur={() => setIcon(icon)}
        onMouseEnter={() => setIcon(alternateIcon)}
        onClick={() => setIcon(alternateIcon)}
        onMouseLeave={() => setIcon(icon)}
        href={path}
        key={label}
      >
        <button
          className={previewClass}
          // className={router.path() == slug ? activeClass : previewClass}
        >
          <span className="size-5">{currentIcon}</span>
          <span className="btm-nav-label text-sm font-medium capitalize">
            {label}
          </span>
        </button>
      </Link>
    </div>
  );
}
