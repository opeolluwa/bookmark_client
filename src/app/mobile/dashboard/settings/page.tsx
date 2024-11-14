"use client";
import Card from "@/components/Cards";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { ChevronRightIcon } from "@heroicons/react/24/outline";
import { Divider } from "antd";
import { settingsTab } from "./uitils";

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
                <div className="my-5">{component}</div>
                {routes?.map((route) => (
                  <ul>
                    <li className="flex justify-between items-center my-5">
                      <View className="flex items-center gap-x-4  px-0 mx-0">
                        <span className="size-5 font-semibold">
                          {route.icon}
                        </span>
                        <span className="text-black">{route.label}</span>
                      </View>
                      <ChevronRightIcon className="size-4 text-gray-500" />
                    </li>
                  </ul>
                ))}
              </View>
            </>
          );
        })}
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Rem, eius
        molestiae voluptas deserunt suscipit sint sed alias assumenda
        perferendis ullam nemo ratione voluptatem qui voluptatibus nobis eum
        nostrum placeat non. Lorem ipsum dolor sit amet consectetur adipisicing
        elit. Sequi eius modi numquam veniam harum soluta ab illo minima,
        placeat voluptates cumque optio, asperiores impedit magnam culpa. Cum
        perferendis saepe voluptatum. Lorem ipsum dolor sit amet, consectetur
        adipisicing elit. Sit excepturi aliquam numquam placeat esse ipsa
        aperiam at, nesciunt earum, molestiae ad tempora dolorem nostrum eum
        rem. Quam alias tenetur dolor.
        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Nesciunt, eius illum doloremque quibusdam aliquid ad maxime aspernatur id. Dolores esse numquam tenetur voluptatem placeat hic nisi maiores rerum est voluptates?
      </Card>
    </>
  );
}
