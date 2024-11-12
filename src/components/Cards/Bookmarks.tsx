import {
  EllipsisHorizontalIcon,
  EllipsisVerticalIcon,
  PencilSquareIcon,
  PlusCircleIcon,
  TrashIcon,
} from "@heroicons/react/24/outline";
import { Dropdown, MenuProps } from "antd";
import Card from ".";
import Heading from "../Heading";
import Text from "../Text";
import { parseDate } from "./utils";
import View from "../View";
import TimeAgo from "javascript-time-ago";
import en from "javascript-time-ago/locale/en";
import ReactTimeAgo from "react-time-ago";
interface Props {
  name: string;
  description: string;
  date: string;
}
export const items: MenuProps["items"] = [
  {
    key: "add",
    label: (
      <span className="flex gap-x-[2px] items-center">
        <PlusCircleIcon className="w-4 h-4 mr-2" />
        Add entry
      </span>
    ),
  },
  {
    key: "edit",
    label: (
      <span className="flex gap-x-[2px] items-center">
        <PencilSquareIcon className="w-4 h-4 mr-2" />
        Edit
      </span>
    ),
  },

  {
    key: "delete",
    danger: true,
    label: (
      <span className="flex gap-x-[2px] items-center">
        <TrashIcon className="w-4 h-4 mr-2" />
        Delete
      </span>
    ),
  },
];
export default function Bookmark({ name, date, description }: Props) {
  // const result = useTimeAgo(date);
  TimeAgo.addDefaultLocale(en);

  return (
    <Card className="border-[2px] relative card w-[180px] h-[180px] border-gray-100 flex  cursor-pointer  px-6 rounded-xl">
      <div className=" ">
        <Heading className="font-semibold text-gray-600 card-title text-xl leading-relaxed">
          {name}
        </Heading>
        <Text className="leading-1">{description}</Text>
      </div>
      <View className="flex text-gray-600 text-sm absolute w-[150px justify-between bottom-4 right-6 left-6">
        <>
          created:{" "}
          <ReactTimeAgo
            date={parseDate(date)}
            locale="en-Us"
            timeStyle="twitter"
          />{" "}
          ago
        </>
        <Dropdown menu={{ items }}>
          <EllipsisHorizontalIcon className="w-6 h-6 cursor-pointer" />
        </Dropdown>
      </View>
    </Card>
  );
}
