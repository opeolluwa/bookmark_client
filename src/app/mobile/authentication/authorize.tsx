"use client";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Form, FormProps, Input } from "antd";
import { useRouter } from "next/navigation";
import { ArrowLongLeftIcon } from "@heroicons/react/24/solid";
import SmallText from "@/components/SmallText";
import { useState } from "react";
import { FormFieldTypes } from "@/app/page";
import { LoginData } from "../../../../tauri/bindings/LoginData";
import Link from "next/link";

export default function AuthorizeWithPassword() {
  const [form] = Form.useForm();
  const router = useRouter();
  const [accountExist, setAccountExist] = useState(false);
  const submit_form: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const new_user: LoginData = {
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    invoke("sign_in", { user: new_user }).then((res) => {
      console.log({ res });
    });
  };
  return (
    <View className="py-14 px-6 relative min-h-screen ">
      <Form
        initialValues={{ remember: true }}
        onFinish={submit_form}
        autoComplete="off"
        name="save-data"
        layout="vertical"
        className=" flex flex-col rounded-lg shadow-gray-300 bg-white py-6 "
        form={form}
      >
        <View className="mb-12 flex justify-between items-center">
          <Link href="/mobile/authentication/sign-up">
            <ArrowLongLeftIcon className="w-6 h-6" />
          </Link>
          <SmallText className="font-medium text-app-600">
            Not Adeoye?
          </SmallText>
        </View>

        <View className="mb-12">
          <Heading className="font-bold">Welcome, Adeoye</Heading>
          <Text className="leading-1 mt-2 text-sm">
            Enter your password to log into your account
          </Text>
        </View>

        <Form.Item<FormFieldTypes> label="Password" name="password">
          <Input.Password
            type="password"
            name="password"
            className="w-full rounded-lg py-4 focus:border-app-500 focus:outline-none border-[2px] bg-white border-gray-300 placeholder:pb-2 px-2 "
            placeholder="enter your password"
            iconRender={(visible) =>
              visible ? <EyeTwoTone /> : <EyeInvisibleOutlined />
            }
          />
        </Form.Item>

        <button
          onClick={() => router.push("/dashboard")}
          className="w-full rounded-lg py-4 bg-app-600 text-white font-medium"
        >
          Sign in
        </button>
      </Form>
      <Link
        href="/mobile/authentication/forgotten-password"
        className=" text-app block text-sm font-bold"
      >
        Forgotten password?
      </Link>
    </View>
  );
}
