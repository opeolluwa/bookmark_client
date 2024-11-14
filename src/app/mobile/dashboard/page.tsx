"use client";
import Bookmark, { BookmarkProps } from "@/components/Cards/Bookmarks";
import Text from "@/components/Text";
import View from "@/components/View";
import { UserOutlined } from "@ant-design/icons";
import { Bars3Icon, PlusIcon } from "@heroicons/react/24/solid";
import { Avatar, FloatButton } from "antd";

let test_data: BookmarkProps[] = [
  {
    name: "Project Alpha",
    description: "A top-priority project for developing an AI-based solution.",
    isStarred: true,
  },
  {
    name: "Research Paper Review",
    description:
      "Review recent papers on machine learning trends and applications.",
  },
  {
    name: "Documentation Update",
    description: "Update the documentation for the latest API changes.",
    isStarred: false,
  },
  {
    name: "Team Sync Meeting",
    description: "Weekly sync meeting with the engineering team.",
  },
  {
    name: "Feature Request",
    description:
      "Investigate adding multi-language support to the application.",
    isStarred: true,
  },
  {
    name: "Code Refactoring",
    description:
      "Refactor the codebase for improved maintainability and readability.",
  },
];

export default function page() {
  return (
    <>
      <View className="mb-6">
        <header className="flex items-center justify-between">
          <Bars3Icon className="w-6 h-6 text-gray-700" />
          <Text className="text-gray-400 font-bold text-sm">
            Default collection
          </Text>
          <Avatar icon={<UserOutlined />} />
        </header>
      </View>
      <View className="mb-6">
        {test_data.map((bookmark) => (
          <Bookmark
            name={bookmark.name}
            description={bookmark.description}
            isStarred={bookmark.isStarred}
            key={bookmark.name}
          />
        ))}
      </View>
      <FloatButton
        shape="circle"
        type="primary"
        className="mr-4 mb-4"
        style={{ insetInlineEnd: 24 }}
        icon={<PlusIcon className="size-4" />}
        // onClick={showModal}
      />
    </>
  );
}
