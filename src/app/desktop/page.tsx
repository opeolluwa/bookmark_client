"use client";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Form, FormProps, Input, message } from "antd";
import { useRouter } from "next/navigation";
import { useState } from "react";
import type { LoginResponse } from "vault_grpc_bindings/bindings";
import { CommandResponse } from "../../../tauri/bindings/CommandResponse";
import { LoginData } from "../../../tauri/bindings/LoginData";
import { FormFieldTypes } from "../page";

export default function DesktopAppEntry() {
  const [form] = Form.useForm();
  const router = useRouter();
  const [processing_request, set_processing_request] = useState(false);
  const [messageApi, contextHolder] = message.useMessage();

  const submit_form: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    set_processing_request(true);
    const login_details: LoginData = {
      email: values.email?.trim().toLowerCase(),
      password: values.password?.trim(),
    };
    try {
      invoke("sign_in", { payload: login_details })
        .then((response) => {
          console.log(login_details);
          const result = response as CommandResponse<LoginResponse>;
          if (result.status != "Success") {
            messageApi.error(result.message);
            return;
          } else {
            set_processing_request(false);
            messageApi.success(result.message || "Login successful");
            router.push("/dashboard");
          }
        })
        .catch((error) => {
          set_processing_request(false);
          return;
        });
    } catch (error) {
      messageApi.error((error as any).message);
      set_processing_request(false);
      return;
    }
  };

  return (
    <View className="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full ">
      {contextHolder}

      <View className="col-span-5 py-8 px-6 w-[40%] ">
        <Form
          initialValues={{ remember: true }}
          onFinish={submit_form}
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

          <Form.Item>
            <button
              disabled={processing_request}
              type="submit"
              onClick={() => submit_form}
              className=" bg-app-600 btn disabled:bg-app-600 disabled:text-white border-white w-full text-white"
            >
              {processing_request ? (
                <span className="loading loading-ring loading-sm"></span>
              ) : (
                "Sign in"
              )}
            </button>
          </Form.Item>

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
