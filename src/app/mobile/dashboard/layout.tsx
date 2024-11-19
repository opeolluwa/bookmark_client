"use client";
import BottomNavItem from "@/components/Navigation/BottomNavItem";
import View from "@/components/View";
import { StarFilled } from "@ant-design/icons";
import {
  Cog6ToothIcon,
  HomeIcon,
  MagnifyingGlassIcon,
  StarIcon,
  UserIcon,
} from "@heroicons/react/24/outline";
import {
  Cog6ToothIcon as SolidCog6ToothIcon,
  HomeIcon as SolidHomeIcon,
  MagnifyingGlassIcon as SolidMagnifyingGlassIcon,
  UserIcon as SolidUserIcon,
} from "@heroicons/react/24/solid";
import React, { ReactNode, useState } from "react";
import { TabBar } from "antd-mobile";
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
    label: "Search",
    icon: <MagnifyingGlassIcon />,
    alternateIcon: <SolidMagnifyingGlassIcon />,
  },
  {
    icon: <StarIcon />,
    alternateIcon: <StarFilled />,
    label: "favorites",
    path: "favorites",
  },
  {
    icon: <Cog6ToothIcon />,
    alternateIcon: <SolidCog6ToothIcon />,
    label: "settings",
    path: "settings",
  },
  {
    icon: <UserIcon />,
    alternateIcon: <SolidUserIcon />,
    label: "profile",
    path: "profile",
  },
];
export default function MobileAppDashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const [activeKey, setActiveKey] = useState("/");

  return (
    <View className="pt-4 pb-12 px-6 h-screen overflow-y-auto ">
      {children}

      <View className="btm-nav  z-50 bg-white/90 fixed rounded-t-3xl text-gray-500 py-4">
        {bottomNavigation.map((route) => {
          const { icon, alternateIcon, label, path: slug } = route;
          return (
            <BottomNavItem
              label={label}
              alternateIcon={alternateIcon}
              icon={icon}
              path={slug}
              key={label}
            />
          );
        })}
      </View>
    </View>
  );
}
