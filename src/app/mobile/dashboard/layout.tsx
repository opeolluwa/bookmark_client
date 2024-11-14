"use client";
import View from "@/components/View";
import { BellIcon, Cog6ToothIcon, UserIcon, HomeIcon, PlusIcon } from "@heroicons/react/24/outline";
import React, { ReactNode } from "react";

interface Route {
  label: string;
  path?: string;
  icon?: ReactNode;
  action?: () => void;
}
const bottom_navigation: Route[] = [
  {
    label: "home",
    path: "/",
    icon: <HomeIcon className="w-5 h-5" />,
  },
  {
    icon: <BellIcon className="w-5 h-5" />,
    label: "notification",
    path: "notification",
  },
  {
    icon: <PlusIcon className="w-8 h-8 font-black" />,
    label: "save",
  },
  {
    icon: <UserIcon className="w-5 h-5" />,
    label: "profile",
    path: "profile",
  },

  {
    icon: <Cog6ToothIcon className="w-5 h-5" />,
    label: "settings",
    path: "settings",
  },
];
export default function MobileAppDashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <View className="py-12 px-6">
      {children}

      <View className="btm-nav rounded-t-xl text-gray-500 py-4 bg-white/30">
        {bottom_navigation.map((route) => (
          <a href={`/mobile/dashboard/${route.path}`} key={route.label}>
            {route.label == "save" ? (
              <button className="flex flex-col justify-center items-center text-white bg-app rounded-full p-1 shadow-sm">
                {route.icon}
               
              </button>
            ) : (
              <button className="flex flex-col justify-center items-center hover:text-app">
                {route.icon}
                <span className="btm-nav-label text-sm font-medium capitalize">
                  {route.label}
                </span>
              </button>
            )}
          </a>
        ))}
      </View>
    </View>
  );
}
