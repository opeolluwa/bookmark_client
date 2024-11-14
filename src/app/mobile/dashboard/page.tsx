"use client";
import Bookmark, { BookmarkProps } from "@/components/Cards/Bookmarks";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { UserOutlined } from "@ant-design/icons";
import {
  ArrowLeftStartOnRectangleIcon,
  Bars3Icon,
  PlusIcon,
} from "@heroicons/react/24/solid";
import {
  Avatar,
  Button,
  Drawer,
  FloatButton,
  Form,
  FormProps,
  Input,
  Space,
} from "antd";
import { useState } from "react";
import { BookmarkCollectionEntries } from "vault_grpc_bindings/bindings";

const { TextArea, Search } = Input;

type FormFieldTypes = {
  title?: string;
  description?: string;
};

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

export default function Page() {
  const [form] = Form.useForm();
  const [openDrawer, setOpenDrawer] = useState(false);
  const [openSideNavigation, setOpenSideNavigation] = useState<boolean>(false);
  const [loadingBookmarks, setLoadingBookmarks] = useState<boolean>(true);
  const [bookmarks, setBookmarks] = useState<BookmarkCollectionEntries[]>();

  const hideDrawer = () => setOpenDrawer(false);
  const showDrawer = () => setOpenDrawer(true);

  const hideSideNavigation = () => setOpenSideNavigation(false);
  const showSideNavigation = () => setOpenSideNavigation(true);

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values });
    form.resetFields();
  };

  return (
    <>
      <View className="mb-6">
        <header className="flex items-center justify-between">
          <Bars3Icon
            className="w-6 h-6 text-gray-700"
            onClick={showSideNavigation}
          />
          <Text className="text-gray-400 font-bold text-sm">
            Default collection
          </Text>
          <Avatar icon={<UserOutlined />} />
        </header>
      </View>

      {!bookmarks?.length ? (
        <View className="flex flex-col justify-center items-center mt-24">
          <img
            src="/illustrations/empty-bookmarks.png"
            alt="empty"
            className="grayscale sepia"
          />
          <SmallText>Such Emptiness!</SmallText>
        </View>
      ) : (
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
      )}

      <FloatButton
        shape="circle"
        type="primary"
        className="mr-6 mb-5"
        style={{ insetInlineEnd: 24 }}
        icon={<PlusIcon className="size-4" />}
        onClick={showDrawer}
      />
      <Drawer
        placement="bottom"
        title="New bookmark"
        onClose={hideDrawer}
        className="rounded-t-2xl"
        open={openDrawer}
        extra={
          <Space>
            <Button type="primary" onClick={hideDrawer}>
              Save
            </Button>
          </Space>
        }
      >
        <Form
          initialValues={{ remember: true }}
          onFinish={submitForm}
          autoComplete="off"
          name="save-data"
          layout="vertical"
          className="my-4 flex flex-col"
          form={form}
        >
          <Form.Item<FormFieldTypes> label="Title" name="title">
            <Input
              autoFocus
              type="text"
              name="title"
              className="w-full rounded py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes> label="Description" name="description">
            <TextArea
              showCount
              maxLength={100}
              name="description"
              style={{ height: 100, resize: "none" }}
            />
          </Form.Item>
          <View></View>
        </Form>
      </Drawer>
      {/**side navigation */}
      <Drawer
        closable
        destroyOnClose
        title={"Collections"}
        placement="left"
        open={openSideNavigation}
        loading={loadingBookmarks}
        height={"80vh"}
        width={"80vw"}
        onClose={() => setOpenSideNavigation(false)}
        footer={
          <div className="flex gap-x-4 py-2 font-medium">
            <ArrowLeftStartOnRectangleIcon className="size-5" /> Logout
          </div>
        }
      ></Drawer>
    </>
  );
}
