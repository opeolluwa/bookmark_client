import React from "react";
import View from "@/components/View";
import Heading from "@/components/Heading";
import { Divider } from "antd";
import Text from "@/components/Text";
export default function Page() {
  return (
    <>
      <Heading>Activity log</Heading>
      <Text> See where and when you data was accessed</Text>
      <Divider />
      <View></View>
    </>
  );
}
