"use client";

import View from "@/components/View";
import Notification from "@/components/Cards/Notification";
import Heading from "@/components/Heading";
import { Key, useState } from "react";
import { Divider, Empty } from "antd";

export default function Page() {
  const [data, setData] = useState<any>();
  if (data) {
    <Heading>Notification</Heading>;
    {
      data.map(
        (notification: { heading: string; body: string; date: string }) => (
          <Notification
            heading={notification.heading}
            body={notification.body}
            date={notification.date}
            key={notification.heading}
          />
        )
      );
    }
  }
  return (
    <>
      <View>
        <Heading>Notification</Heading>
        <Divider />
      </View>

      <View className="flex items-center justify-center">
        <Empty description="such emptiness" />
      </View>
    </>
  );
}
