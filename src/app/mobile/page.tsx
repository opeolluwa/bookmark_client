"use client";
import { FormFieldTypes } from "@/app/page";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { FingerPrintIcon } from "@heroicons/react/24/outline";
import { ArrowLongLeftIcon } from "@heroicons/react/24/solid";
import { invoke } from "@tauri-apps/api/core";
import { authenticate, checkStatus } from "@tauri-apps/plugin-biometric";
import { message } from "@tauri-apps/plugin-dialog";
import { Form, FormProps, Input } from "antd";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { LoginData } from "../../../tauri/bindings/LoginData";
import { fetch } from "@tauri-apps/plugin-http";

export default function LoginWithEmail() {
  const [accountExist, setAccountExist] = useState(true);
  const [biometricsIsSupported, setBiometricsIsSupported] = useState(false);
  const [form] = Form.useForm();
  const router = useRouter();
  const submit_form: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const new_user: LoginData = {
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    invoke("sign_in", { user: new_user }).then((res) => {
      console.log({ res });
    });
  };

  const authenticateWithBiometrics = async () => {
    const options = {
      allowDeviceCredential: true,
      cancelTitle: "Feature won't work if Canceled",
      fallbackTitle: "Sorry, authentication failed",
      title: "Continue with fingerprint",
      subtitle: "Authenticate to access the locked Tauri function",
      confirmationRequired: true,
    };
    try {
      await authenticate("Continue with fingerprint", options);
      router.push("/mobile/dashboard");
    } catch (error) {
      await message("Biometrics login failed, please email and password", {
        title: "Bookmark",
        kind: "error",
      });
    }
  };
  useEffect(() => {
    const fetchData = async () => {
      const biometricsSupportStatus = await checkStatus();
      if (biometricsSupportStatus.isAvailable) {
        setBiometricsIsSupported(true);
      } else {
        setBiometricsIsSupported(false);
      }
    };
    fetchData();
  });
  return (
    <View className="py-14 px-6 relative h-screen overflow-auto w-full">
      {accountExist ? (
        <View className="mb-12 flex justify-between items-center">
          <Link href="/mobile/authentication/sign-up">
            <ArrowLongLeftIcon className="w-6 h-6" />
          </Link>
          <button
            onClick={() => setAccountExist(false)}
            className="font-medium text-sm  text-gray-600"
          >
            Not Adeoye?
          </button>
        </View>
      ) : (
        <View className="mb-12 flex justify-between items-center">
          <button onClick={() => setAccountExist(true)}>
            <ArrowLongLeftIcon className="w-6 h-6" />
          </button>
        </View>
      )}
      {accountExist ? (
        <View className="mb-6">
          <Heading className="font-bold">Welcome, Adeoye</Heading>
          <Text className="leading-1 mt-2 text-sm">
            Enter your password to log into your account
          </Text>
        </View>
      ) : (
        <View className="mb-6">
          <Heading className="font-bold">Log in</Heading>
          <Text className="leading-1 mt-2 text-sm">
            Enter your email and password to log in
          </Text>
        </View>
      )}

      <Form
        initialValues={{ remember: true }}
        onFinish={submit_form}
        autoComplete="off"
        name="save-data"
        layout="vertical"
        className=" flex flex-col rounded-lg shadow-gray-300 pb-6"
        form={form}
      >
        {!accountExist && (
          <Form.Item<FormFieldTypes> label="Email" name="email">
            <Input
              type="email"
              name="email"
              className="w-full rounded-lg py-4 focus:border-app-500 focus:outline-none border-[2px] bg-white border-gray-300 placeholder:pb-2 px-2 "
              placeholder="enter your email"
            />
          </Form.Item>
        )}

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
          onClick={() => router.push("/mobile/dashboard")}
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

      {biometricsIsSupported && (
        <div className="flex items-center justify-center absolute bottom-24 left-0 right-0">
          <button
            className="btn bg-app-50 border-none  "
            onClick={authenticateWithBiometrics}
          >
            <FingerPrintIcon className="size-6 text-app" />
          </button>
        </div>
      )}
    </View>
  );
}
