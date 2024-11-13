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
     
    </>
  );
}
