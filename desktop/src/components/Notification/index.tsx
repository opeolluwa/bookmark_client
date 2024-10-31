"use client";

import React from "react";
import Card from "../Card";
import Heading from "../Heading";
import Text from "../Text";
import SmallText from "../SmallText";

export interface Props  {
  heading: string;
  body: string;
  date: string;
  className?: string;
}


export default function Component({ heading, body, date, className }: Props) {
  return (
    <Card
      className={
        "cursor-pointer bg-gray-0 rounded border-gray-100 my-4 border-[2px] first:mt-0 px-6 " +
        className
      }
    >
      <Heading className="text-[14px]">{heading}</Heading>
      <Text className="text-gray-500 font-sm my-1 leading-1 ">{body}</Text>
      <SmallText className="text-sm text-gray-500 mt-1 text-[14px]">
        {date}
      </SmallText>
    </Card>
  );
}

