"use client";
import Bookmark from "@/components/Cards/Bookmarks";
import Heading from "@/components/Heading";
import View from "@/components/View";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import type { FormProps } from "antd";
import { Button, Empty, Form, Input, Modal, notification } from "antd";
import Card from "@/components/Cards";
import { useEffect, useState } from "react";
import type {
  ListVaultsRequest,
  ListVaultsResponse,
  NewBookmarkCollectionRequest,
  NewVaultResponse,
} from "vault_grpc_bindings/bindings";
import { CommandResponse, FormFieldTypes, TextArea } from "./utils";

type NotificationType = "success" | "info" | "warning" | "error";

export default function Home() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [vault, setVaults] = useState<ListVaultsResponse>();
  const [form] = Form.useForm();
  const [api, contextHolder] = notification.useNotification();

  const openNotification = ({
    message,
    kind,
    description,
  }: {
    message: string;
    description: string;
    kind: NotificationType;
  }) => {
    api[kind]({
      message,
      description,
      role: "status",
      duration: 0,
    });
  };

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = async (values) => {
    const payload: NewBookmarkCollectionRequest = {
      name: String(values.name?.trim()),
      description: String(values?.description?.trim()),
    };
    const response = (await invoke("create_bookmark_collection", {
      payload,
    }).catch(async (error) => {
      await message(error.message, { title: "New collection", kind: "error" });
    })) as unknown as CommandResponse<NewVaultResponse>;
    openNotification({
      message: response.message || "Collection created successfully",
      kind: "success",
      description: "",
    });

    // await message("collection created successfully", {
    //   title: "New collection",
    //   kind: "info",
    // });
    setIsModalOpen(false);
    form.resetFields();
  };

  const submitFormFailed: FormProps<FormFieldTypes>["onFinishFailed"] = (
    errorInfo
  ) => {
    console.log("Failed:", errorInfo);
  };

  const showModal = () => {
    setIsModalOpen(true);
  };
  const handleOk = () => {
    setIsModalOpen(false);
    form.submit();
  };
  const handleCancel = () => {
    setIsModalOpen(false);
    form.resetFields();
  };

  useEffect(() => {
    const payload: ListVaultsRequest = {
      page: 0,
      page_size: 5,
    };
    invoke("get_all_bookmark_collections", {
      payload,
    })
      .then((response) => {
        let result = response as unknown as CommandResponse<ListVaultsResponse>;
        console.log(result);
        setVaults(result.body as ListVaultsResponse);
      })
      .catch((error) => {
        console.log(error.message);
      });
  }, []);

  return (
    <>
      {contextHolder}
      <View className="my-6 relative">
        <View className="flex justify-between items-center">
          <Heading>Collections</Heading>

          <Button
            className="w-fit hover:bg-black bg-app-600 shadow text-white border-none"
            onClick={showModal}
          >
            Create new
          </Button>
        </View>
      </View>

      <View className="flex  flex-wrap gap-6">
        {vault?.vaults.length == 0 ? (
          <Card></Card>
        ) : (
          vault?.vaults.map((entry) => (
            <Bookmark
              name={entry.name}
              description={entry.description}
              key={entry.vault_id}
              title={""}
              created_at={""}
              updated_at={""}
              bookmark_collection_id={""}
              more_fields={{}}
            />
          ))
        )}
      </View>

      <View className="h-screen">
        <Modal
          title="New Collection"
          open={isModalOpen}
          onOk={handleOk}
          centered
          onCancel={handleCancel}
          afterClose={() => submitForm}
          footer={[
            <Button key="back" onClick={handleCancel}>
              Cancel
            </Button>,
            <Button
              key="submit"
              type="primary"
              htmlType="submit"
              onClick={handleOk}
            >
              Submit
            </Button>,
          ]}
        >
          <Form
            initialValues={{ remember: true }}
            onFinish={submitForm}
            onFinishFailed={submitFormFailed}
            autoComplete="off"
            name="save-data"
            layout="vertical"
            className="my-4 flex flex-col gap-y-"
            form={form}
          >
            <Form.Item<FormFieldTypes>
              label="Name"
              name="name"
              rules={[
                {
                  required: true,
                  message: "Please input the collection name!",
                },
              ]}
            >
              <Input
                autoFocus
                type="text"
                name="name"
                className="w-full rounded py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              />
            </Form.Item>
            <Form.Item<FormFieldTypes>
              label="Description"
              name="description"
              rules={[
                { required: true, message: "Please input the description" },
              ]}
            >
              <TextArea
                showCount
                maxLength={100}
                name="description"
                style={{ height: 120, resize: "none" }}
              />
            </Form.Item>
          </Form>
        </Modal>
      </View>
    </>
  );
}
