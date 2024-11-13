import React from "react";
import View from "@/components/View";
import Text from "@/components/Text";
import Heading from "@/components/Heading";
import { Divider } from "antd";
export default function Page() {
  return (
    <>
      <Heading>Settings</Heading>
      <Text> Manage your application settings</Text>
      <Divider />
      <View></View>
    </>
  );
}
