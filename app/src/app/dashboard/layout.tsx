"use client";
import { AppLogo } from "@/components/App/AppLogo";
import { routes } from "@/components/Navigation";
import { Breadcrumb, Layout, Menu, MenuProps, theme } from "antd";
import React, { useState } from "react";
const { Header, Content, Sider } = Layout;

interface Props {
  children: React.ReactNode;
}

export default function DashboardLayout({ children }: Props) {
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();
  const [collapsed, setCollapsed] = useState(true);

  const items1: MenuProps["items"] = ["1", "2", "3"].map((key) => ({
    key,
    label: `nav ${key}`,
  }));


  return (
    <Layout className="h-screen overflow-hidden">
      <Header
        style={{
          display: "none",
          alignItems: "center",
          justifyContent: "space-between",
        }}
      >
        <AppLogo className="w-[35px] hidden" />
        <Menu
          theme="dark"
          mode="horizontal"
          defaultSelectedKeys={["2"]}
          items={items1}
          style={{ display: "hidden", backgroundColor: "inherit" }}
        />
      </Header>
      <Layout>
        <Sider
          width={250}
          style={{ background: colorBgContainer }}
          className="h-screen bg-gray-50 overflow-hidden "
          collapsible
          collapsed={collapsed}
          onCollapse={(value) => setCollapsed(value)}
        >
          <Menu
            theme="dark"
            defaultSelectedKeys={["dashboard"]}
            mode="inline"
            items={routes}
            style={{ height: "100%", borderRight: 0 }}
          />
        </Sider>
        <Layout style={{ padding: "24px" }}>
          <Breadcrumb
            items={[
              { title: "Adeoye" },
              { title: "Vaults" },
              { title: "Default" },
            ]}
            className="hidden"
            style={{ margin: "16px 0" }}
          />
          <Content
            style={{
              padding: 24,
              margin: 0,
              minHeight: 300,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            {children}
          </Content>
        </Layout>
      </Layout>
    </Layout>
  );
}
