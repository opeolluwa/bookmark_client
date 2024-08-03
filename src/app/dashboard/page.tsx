import Button from "@/components/Button";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import AppLayout from "@/Layouts/AppLayout";
import React from "react";

export default function Home() {
  return (
    <AppLayout>
      <View className="grid grid-cols-12 gap-2">
        <View className=" col-span-8 border-r-2 h-screen ">
          <View>
            <Heading>Hello Opeoluwa!</Heading>
            <Text className="mt-2">
              Track team progress here do whatever you like{" "}
            </Text>
          </View>
        </View>
        <View className=" col-span-4"></View>
      </View>
    </AppLayout>
  );
}
