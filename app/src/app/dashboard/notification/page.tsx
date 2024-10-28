"use client";

import View from "@/components/View";
import Notification from "@/components/Notification";
import Heading from "@/components/Heading";

const notifications = [
  {
    heading: "Introduction to JavaScript",
    body: "Learn the basics of JavaScript, the most popular programming language for web development.",
    date: "2023-10-15",
  },
  {
    heading: "React vs. Vue: A Comparison",
    body: "An in-depth look at two of the most popular frontend frameworks and their use cases.",
    date: "2023-11-02",
  },
  {
    heading: "Understanding Async/Await",
    body: "A guide on asynchronous JavaScript programming using async/await syntax.",
    date: "2023-09-28",
  },
  {
    heading: "CSS Grid Layout",
    body: "Master the CSS Grid layout system to create responsive web designs easily.",
    date: "2023-12-01",
  },
  {
    heading: "Building REST APIs with Node.js",
    body: "A comprehensive tutorial on building scalable REST APIs using Node.js and Express.",
    date: "2024-01-10",
  },
  {
    heading: "Introduction to TypeScript",
    body: "An overview of TypeScript, its benefits, and how to use it with JavaScript projects.",
    date: "2024-02-05",
  },
];

export default function Page() {
  return (
    <>
      <View>
        <Heading>Notification</Heading>
        {notifications.map((notification) => (
          <Notification
            heading={notification.heading}
            body={notification.body}
            date={notification.date}
          />
        ))}
      </View>
    </>
  );
}
