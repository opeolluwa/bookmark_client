"use client";
import View from "@/components/View";
import { BellIcon, Cog6ToothIcon, HomeIcon, PlusIcon, UserIcon } from "@heroicons/react/24/outline";
import React, { ReactNode } from "react";

export interface ApplicationRoute {
  label: string;
  path?: string;
  icon?: ReactNode;
  action?: () => void;
}
const bottomNavigation: ApplicationRoute[] = [
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
    <View className="py-12 px-6 min-h-screen">
      {children}

      <View className="btm-nav z-50 bg-white rounded-t-xl text-gray-500 py-4">
        {bottomNavigation.map((route) => (
          <a href={`/mobile/dashboard/${route.path}`} key={route.label}>
            {route.label == "save" ? (
              <button className="flex flex-col justify-center items-center text-white bg-app rounded-full p-1 mb-8 shadow-sm">
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
