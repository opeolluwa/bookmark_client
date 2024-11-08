"use client";
import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Form, FormProps, Input } from "antd";
import { useEffect, useState } from "react";
import { LoginData } from "../../tauri/bindings/LoginData";
import { OsType, type } from "@tauri-apps/plugin-os";

type FormFieldTypes = {
  email: string;
  password: string;
};

import React from "react";

export default function LoginPage() {
  const [osType, setOsType] = useState<OsType>("" as OsType);
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const fetchData = () => {
      const os = type();
      setOsType(os);

      if (os == "android" || os == "ios") {
        setIsMobile(true);
      }
    };

    fetchData();
  }, []);

  if (isMobile) {
    return <MobileAppEntry />;
  } else {
    return <DesktopAppEntry />;
  }
}

function DesktopAppEntry() {
  const [form] = Form.useForm();
  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const login_details: LoginData = {
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    alert(login_details.email);
    invoke("sign_in", { user: login_details }).then((res) => {
      console.log({ res });
    });
  };

  return (
    <View className="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] [background-size:16px_16px] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]">
      <View className="col-span-5 py-8 px-6 w-[40%] ">
        <Form
          initialValues={{ remember: true }}
          onFinish={submitForm}
          autoComplete="off"
          name="save-data"
          layout="vertical"
          className="my-4 mt-12 flex flex-col rounded-lg shadow-lg shadow-gray-300 bg-white px-8 py-6 "
          form={form}
        >
          <View className="text-center mb-3">
            <Heading className="font-semibold">Welcome back!</Heading>
            <Text className="leading-1">Sign in to continue</Text>
          </View>
          <Form.Item<FormFieldTypes>
            label="Email"
            name="email"
            rules={[{ required: true, message: "email is required!" }]}
          >
            <Input
              autoFocus
              type="email"
              name="email"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes>
            label="Password"
            name="password"
            rules={[{ required: true, message: "password is required!" }]}
          >
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
            href="/dashboard"
            className=" bg-app-600 text-center w-full py-2 text-white"
          >
            Sign in
          </Button>
          <SmallText
            href="/authentication/password-reset"
            className="text-right  underline mt-4"
          >
            Forgotten password?
          </SmallText>
        </Form>
        <SmallText className="text-center mt-4">
          Don&apos;t have an account?
          <a className="text-app-600 ml-2" href="/authentication/signup">
            Sign up
          </a>
        </SmallText>
      </View>
    </View>
  );
}

function MobileAppEntry() {
  const [form] = Form.useForm();
  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const new_user: LoginData = {
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    invoke("sign_in", { user: new_user }).then((res) => {
      console.log({ res });
    });
  };
  return (
    <View className=" pt-4 px-6">
      <Form
        initialValues={{ remember: true }}
        onFinish={submitForm}
        autoComplete="off"
        name="save-data"
        layout="vertical"
        className=" flex flex-col rounded-lg shadow-gray-300 bg-white py-6 "
        form={form}
      >
        <View className="mb-12">
          <Heading className="font-semibold">Welcome back!</Heading>
          <Text className="leading-1">Sign in to continue</Text>
        </View>
        <Form.Item<FormFieldTypes>
          label="Email"
          name="email"
          rules={[{ required: true, message: "email is required!" }]}
        >
          <Input
            autoFocus
            type="email"
            name="email"
            className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2"
          />
        </Form.Item>
        <Form.Item<FormFieldTypes>
          label="Password"
          name="password"
          rules={[{ required: true, message: "password is required!" }]}
        >
          <Input.Password
            type="password"
            name="password"
            className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 placeholder:pb-2 px-2 "
            iconRender={(visible) =>
              visible ? <EyeTwoTone /> : <EyeInvisibleOutlined />
            }
          />
        </Form.Item>

        <Button
          href="/dashboard"
          className=" bg-app-600 text-center w-full py-2 text-white"
        >
          Sign in
        </Button>
        <SmallText
          href="/authentication/password-reset"
          className="text-gray-500 font-medium no-underline text-sm mt-4"
        >
          Forgotten password?
        </SmallText>
      </Form>
    </View>
  );
}
