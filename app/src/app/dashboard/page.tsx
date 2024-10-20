"use client";
import Button from "@/components/Button";
import { Greeting } from "@/components/Greetings";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import View from "@/components/View";
import AppLayout from "@/Layouts/AppLayout";
import user from "@/store/user";
import { PlusIcon } from "@heroicons/react/24/solid";
import { Input, InputRef, Modal, Switch } from "antd";
import { SearchProps } from "antd/es/input";
import { useEffect, useRef, useState } from "react";
import { TagsInput } from "react-tag-input-component";
import { AudioOutlined } from "@ant-design/icons";
import {
  ChevronDownIcon,
  MagnifyingGlassIcon,
} from "@heroicons/react/24/outline";
import { items } from "@/store/vault";
import Card from "@/components/Card";

const { TextArea, Search } = Input;

export default function Home() {
  const [greeting, setGreeting] = useState("");
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [keywords, setKeywords] = useState([]);
  const [inputVisible, setInputVisible] = useState(false);
  const [inputValue, setInputValue] = useState("");
  const inputRef = useRef<InputRef>(null);
  const [showAdvancedOptions, setShowAdvancedOptions] = useState(false);

  const onSearch: SearchProps["onSearch"] = (value, _e, info) =>
    console.log(info?.source, value);

  const showModal = () => {
    setIsModalOpen(true);
  };
  const handleOk = () => {
    setIsModalOpen(false);
  };
  const handleCancel = () => {
    setIsModalOpen(false);
  };

  useEffect(() => {
    if (inputVisible) {
      inputRef.current?.focus();
    }
  }, [inputVisible]);

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
        <View className="flex justify-between items-center mb-8">
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
          <Button
            className="bg-app cursor-pointer gap-x-4 flex py-0 items-center  col-span-3 rounded-r"
            onClick={showModal}
          >
            <PlusIcon className="w-6 h-6 px-[1px] text-white"></PlusIcon>

            <SmallText className="text-white">Add new</SmallText>
          </Button>
        </View>
        <Modal
          title="Save to vault"
          open={isModalOpen}
          onOk={handleOk}
          centered
          onCancel={handleCancel}
          okText={"Save"}
        >
          <form className="my-4 flex flex-col gap-y-4">
            <input
              autoFocus
              type="text"
              className="w-full rounded py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              placeholder="title"
            />
            <TextArea
              className="m"
              maxLength={100}
              placeholder="description"
              style={{ height: 120, resize: "none" }}
            />
            <TagsInput
              value={keywords}
              placeHolder="new tag"
              classNames={{
                input:
                  "w-full rounded p-2 focus:border-app-500 focus:outline-none border-2 bg-white border-gray-300 block",
              }}
            />
            {showAdvancedOptions && <>advanced options</>}
            <SmallText>
              <Switch
                className="w-10"
                onChange={() => setShowAdvancedOptions(!showAdvancedOptions)}
              ></Switch>
              {" More fields "}
            </SmallText>
          </form>
        </Modal>

        <View>
          {items.map((item, key) => (
            <Card
              key={key}
              className="my-4 first: mt-6 cursor-pointer flex justify-between items-center bg-gray-50"
            >
              {item.title} <ChevronDownIcon className="w-6 h-6" />{" "}
            </Card>
          ))}
        </View>
      </View>
    </AppLayout>
  );
}
