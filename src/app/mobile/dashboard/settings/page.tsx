"use client";
import Card from "@/components/Cards";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { ChevronRightIcon } from "@heroicons/react/24/outline";
import { Divider } from "antd";
import { settingsTab } from "./uitils";
import Link from "next/navigation";

export default function Page() {
  return (
    <>
      <Card className="">
        <Heading>Settings</Heading>
        <Text> Manage your application settings</Text>
        <Divider />
        {settingsTab.map((item) => {
          const { title, routes, component } = item;
          return (
            <>
              <View
                className="my-12 first:mt-6 last:mb-6"
                key={title?.toString()}
              >
                <Text className="text-sm font-medium capitalize">
                  {" "}
                  {title}{" "}
                </Text>
                <div className="my-2">{component}</div>
                {routes?.map((route) => (
                  <ul key={route.path}>
                    <li
                      className="flex justify-between items-center my-5"
                      key={route.path}
                    >
                      <a
                        href={route?.path}
                        className="flex items-center gap-x-4  px-0 mx-0"
                      >
                        <span className="size-5 font-semibold">
                          {route.icon}
                        </span>
                        <span className="text-black">{route.label}</span>
                      </a>
                      <ChevronRightIcon className="size-4 text-gray-500" />
                    </li>
                  </ul>
                ))}
              </View>
            </>
          );
        })}
      </Card>
    </>
  );
}
