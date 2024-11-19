"use client";
import Bookmark, { BookmarkProps } from "@/components/Cards/Bookmarks";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { UserOutlined } from "@ant-design/icons";
import { BellIcon } from "@heroicons/react/24/outline";
import {
  ArrowLeftStartOnRectangleIcon,
  Bars3Icon,
  PlusIcon,
} from "@heroicons/react/24/solid";
import {
  Avatar,
  Badge,
  Button,
  Drawer,
  FloatButton,
  Form,
  FormProps,
  Input,
  Space,
} from "antd";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { BookmarkCollectionEntries } from "vault_grpc_bindings/bindings";

const { TextArea, Search } = Input;

type FormFieldTypes = {
  title?: string;
  description?: string;
};

export default function Page() {
  const [form] = Form.useForm();
  const [openDrawer, setOpenDrawer] = useState(false);
  const [openSideNavigation, setOpenSideNavigation] = useState<boolean>(false);
  const [loadingBookmarks, setLoadingBookmarks] = useState<boolean>(true);
  const [bookmarks, setBookmarks] = useState<BookmarkCollectionEntries[]>();
  const router = useRouter();

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
          <Badge count={5} size="small">
            <BellIcon className="size-5" />
          </Badge>
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
          {bookmarks.map((bookmark) => (
            <Bookmark
              description={bookmark.description}
              title={""}
              created_at={""}
              updated_at={""}
              bookmark_collection_id={""}
              more_fields={{}}
              key={bookmark.bookmark_collection_id}
              name={""}
            />
          ))}
        </View>
      )}

      <FloatButton
        shape="circle"
        type="primary"
        className="mb-8"
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
          className=""
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
        // loading={loadingBookmarks}
        height={"70vh"}
        width={"80vw"}
        onClose={() => setOpenSideNavigation(false)}
        className="rounded-br-xl"
        footer={
          <div
            className="flex gap-x-4 py-2 font-medium"
            onClick={() => router.push("/mobile")}
          >
            <ArrowLeftStartOnRectangleIcon className="size-5" /> Logout
          </div>
        }
      >
        <button className="btn flex justify-center w-full border-app text-app bg-transparent">
          <PlusIcon className="size-4" /> <span>New collection</span>
        </button>
      </Drawer>
    </>
  );
}
