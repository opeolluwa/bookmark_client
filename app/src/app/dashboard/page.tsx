"use client";
import AppButton from "@/components/Button";
import Card from "@/components/Card";
import { Greeting } from "@/components/Greetings";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import View from "@/components/View";
import AppLayout from "@/Layouts/AppLayout";
import VaultEntry from "@/lib/VaultEntry";
import user from "@/store/user";
import DefaultVault from "@/store/vault";
import {
  BellIcon,
  ChevronDownIcon,
  MagnifyingGlassIcon,
} from "@heroicons/react/24/outline";
import { PlusIcon } from "@heroicons/react/24/solid";
import type { FormProps } from "antd";
import { Button, Form, Input, Modal, notification, Switch } from "antd";
import { SearchProps } from "antd/es/input";
import { useEffect, useState } from "react";
import { TagsInput } from "react-tag-input-component";

const { TextArea, Search } = Input;

type FormFieldTypes = {
  title?: string;
  description?: string;
};

export default function Home() {
  const [greeting, setGreeting] = useState("");
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [keywords, setKeywords] = useState<Array<string>>([]);
  const [vaultEntries, setVaultEntries] = useState<Array<VaultEntry>>(
    DefaultVault.content
  );
  const [api, contextHolder] = notification.useNotification();
  const [showAdvancedOptions, setShowAdvancedOptions] = useState(false);
  const [form] = Form.useForm();
  const onSearch: SearchProps["onSearch"] = (value, _e, info) =>
    console.log(info?.source, value);

  const openNotification = () =>
    api.open({
      message: "entry saved!",
      description:
        "I will never close automatically. This is a purposely very very long description that has many many characters and words.",
      duration: 10,
      placement: "bottomRight",
    });

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values, keywords });
    const vault_entry = new VaultEntry(
      String(values.title),
      String(values.description),
      keywords
    );

    DefaultVault.add_entry(vault_entry);
    setVaultEntries(DefaultVault.content);
    console.log(JSON.stringify(DefaultVault.content));
    setIsModalOpen(false);
    // openNotification();
    form.resetFields();
    setKeywords([]);
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
    let text = new Greeting().msg;
    const punctuation = /[.,!?]$/;

    if (punctuation.test(text)) {
      text = text; // If it ends with punctuation, return as-is
    } else {
      text = text + ","; // If it doesn't, add a comma
    }
    setGreeting(text);
  }, []);

  return (
    <AppLayout>
      <View className=" h-screen ">
        <View className="flex justify-between items-start mb-8">
          <div>
            <Heading>
              {greeting} {user.name}
            </Heading>
            <SmallText className="text-white">
              It&apos;s{" "}
              {new Date().toLocaleDateString("en-US", {
                weekday: "long",
                month: "short",
                year: "numeric",
                day: "numeric",
              })}
            </SmallText>
          </div>
          <BellIcon className="w-6 h-6"></BellIcon>
        </View>

        <View className="grid grid-cols-12 mt-12">
          <View className="flex items-center rounded-l border-gray-300 bg-gray-50 border rounded px-2 justify-start col-span-9">
            <MagnifyingGlassIcon className="w-6 h-8"></MagnifyingGlassIcon>
            <input
              autoFocus
              type="search"
              className="w-full py-3 focus:border-app focus:outline-none border border-none block placeholder:pb-2 px-2 "
              placeholder="search"
            />
          </View>
          <AppButton
            className="bg-app cursor-pointer gap-x-4 flex py-0 items-center  col-span-3 rounded-r"
            onClick={showModal}
          >
            <PlusIcon className="w-6 h-6 px-[1px] text-white"></PlusIcon>

            <SmallText className="text-white">Add new</SmallText>
          </AppButton>
        </View>
        <Modal
          title="Save to vault"
          open={isModalOpen}
          onOk={handleOk}
          centered
          onCancel={handleCancel}
          afterClose={() => submitForm}
          footer={[
            <Button key="back" onClick={handleCancel}>
              Cancel
            </Button>,
            <Button key="submit" type="primary" onClick={handleOk}>
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
              label="Title"
              name="title"
              rules={[{ required: true, message: "Please input the title!" }]}
            >
              <Input
                autoFocus
                type="text"
                name="title"
                className="w-full rounded py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              />
            </Form.Item>
            <Form.Item<FormFieldTypes>
              label="Description"
              name="description"
              rules={[
                { required: true, message: "Please input the description!" },
              ]}
            >
              <TextArea
                showCount
                maxLength={100}
                name="description"
                style={{ height: 120, resize: "none" }}
              />
            </Form.Item>
            <View>
              <label className="mb-2 block text-sm">Keywords</label>
              <TagsInput
                value={keywords}
                onChange={setKeywords}
                name="keywords"
                separators={["Enter", "Space"]}
                classNames={{
                  input:
                    "w-full rounded p-2 focus:border-app-500 focus:outline-none border-2 bg-white border-gray-300 block w-full w-100",
                }}
                placeHolder="add tags"
              />
            </View>
            <SmallText>
              <Switch
                className="w-10"
                onChange={() => setShowAdvancedOptions(!showAdvancedOptions)}
              ></Switch>
              {" Add custom fields "}
            </SmallText>

            {/* {showAdvancedOptions && <>advanced options</>} */}
          </Form>
        </Modal>

        <View>
          {DefaultVault.content.length == 0 ? (
            <Card className="flex flex-col items-center justify-center">
              <img
                src="/assets/message.png"
                alt="empty vault"
                className="w-[400px] block grayscale"
              />
              <SmallText>There&apos;s nothing to see here</SmallText>
            </Card>
          ) : (
            vaultEntries.map((item, key) => (
              <Card
                key={key}
                className="my-4 first: mt-6 cursor-pointer flex justify-between items-center bg-gray-50"
              >
                {item.title} <ChevronDownIcon className="w-6 h-6" />{" "}
              </Card>
            ))
          )}
        </View>
      </View>
    </AppLayout>
  );
}
