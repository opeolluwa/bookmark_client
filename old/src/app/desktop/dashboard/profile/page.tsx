"use client";
// import Button from "@/components/Button";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import {
  EyeInvisibleOutlined,
  EyeTwoTone,
  LoadingOutlined,
  PlusOutlined,
} from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import {
  Button,
  Divider,
  Form,
  GetProp,
  Input,
  message,
  Switch,
  Upload,
  UploadProps,
} from "antd";
import { FormProps } from "antd/es/form";
import { User } from "../../../../../tauri/bindings/User";
import { useState } from "react";
type FormFieldTypes = {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  acceptEULA?: boolean;
};

type FileType = Parameters<GetProp<UploadProps, "beforeUpload">>[0];

const getBase64 = (img: FileType, callback: (url: string) => void) => {
  const reader = new FileReader();
  reader.addEventListener("load", () => callback(reader.result as string));
  reader.readAsDataURL(img);
};

export default function Page() {
  const [form] = Form.useForm();
  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const new_user: User = {
      firstName: values.firstName?.trim(),
      lastName: values.lastName?.trim(),
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    invoke("sign_up", { user: new_user }).then((res) => {
      console.log({ res });
    });
  };
  const [loading, setLoading] = useState(false);
  const [imageUrl, setImageUrl] = useState<string>();

  const handleChange: UploadProps["onChange"] = (info) => {
    if (info.file.status === "uploading") {
      setLoading(true);
      return;
    }
    if (info.file.status === "done") {
      // Get this url from response in real world.
      getBase64(info.file.originFileObj as FileType, (url) => {
        setLoading(false);
        setImageUrl(url);
      });
    }
  };

  const uploadButton = (
    <button style={{ border: 0, background: "none" }} type="button">
      {loading ? <LoadingOutlined /> : <PlusOutlined />}
      <div style={{ marginTop: 8 }}>Upload</div>
    </button>
  );

  const beforeUpload = (file: FileType) => {
    const isJpgOrPng = file.type === "image/jpeg" || file.type === "image/png";
    if (!isJpgOrPng) {
      message.error("You can only upload JPG/PNG file!");
    }
    const isLt2M = file.size / 1024 / 1024 < 2;
    if (!isLt2M) {
      message.error("Image must smaller than 2MB!");
    }
    return isJpgOrPng && isLt2M;
  };

  return (
    <>
      <Heading>Profile</Heading>
      <Text> Manage your profile settings</Text>
      <Divider />
      <View className="grid grid-cols-12 gap-x-12 items-center">
        <View className="col-span-6">
          <Heading className="font-normal">Basic info</Heading>
          <Text> Update your bio information</Text>

          <Form
            initialValues={{ remember: true }}
            onFinish={submitForm}
            autoComplete="off"
            name="save-data"
            layout="vertical"
            className="flex flex-col  py-6 "
            form={form}
          >
            <Form.Item<FormFieldTypes> label="First name" name="firstName">
              <Input
                autoFocus
                type="text"
                name="firstName"
                className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              />
            </Form.Item>
            <Form.Item<FormFieldTypes> label="Last name" name="lastName">
              <Input
                autoFocus
                type="text"
                name="title"
                className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              />
            </Form.Item>
            <Form.Item<FormFieldTypes> label="Email" name="email">
              <Input
                autoFocus
                type="email"
                name="email"
                className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              />
            </Form.Item>
            <Form.Item<FormFieldTypes> label="Password" name="password">
              <Input.Password
                autoFocus
                type="password"
                name="password"
                className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 placeholder:pb-2 px-2 "
                iconRender={(visible) =>
                  visible ? <EyeTwoTone /> : <EyeInvisibleOutlined />
                }
              />
            </Form.Item>

            <Button
              onClick={() => submitForm}
              className=" bg-app-600 text-center w-full py-6 text-white"
            >
              Update profile
            </Button>
          </Form>
        </View>

        <View className="col-span-6 hidden">
          <Upload
            name="avatar"
            listType="picture-circle"
            className="avatar-uploader"
            showUploadList={false}
            action="https://660d2bd96ddfa2943b33731c.mockapi.io/api/upload"
            beforeUpload={beforeUpload}
            onChange={handleChange}
            rootClassName="mb-5"
          >
            {imageUrl ? (
              <img src={imageUrl} alt="avatar" style={{ width: "100%" }} />
            ) : (
              uploadButton
            )}
          </Upload>

          <Heading className="font-normal">Two-factor authentication</Heading>
          <Text> Add an extra layer of security to your data</Text>
          <View className="my-3 flex gap-3 ">
            <Text>Enable 2FA</Text> <Switch />
          </View>
        </View>
      </View>
    </>
  );
}
