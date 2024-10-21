import Vault from "@/lib/Vault";
import VaultEntry from "@/lib/VaultEntry";

export const items = [
  // {
  //   title: "Introduction to JavaScript",
  //   description:
  //     "A beginner-friendly guide to learning JavaScript, one of the most popular programming languages.",
  //   keywords: ["JavaScript", "programming", "beginner", "web development"],
  // },
  // {
  //   title: "Mastering React",
  //   description:
  //     "An advanced course on React, a JavaScript library for building user interfaces.",
  //   keywords: ["React", "JavaScript", "frontend", "UI development"],
  // },
  // {
  //   title: "Node.js for Backend Development",
  //   description:
  //     "Learn how to use Node.js to build powerful backend applications and APIs.",
  //   keywords: ["Node.js", "backend", "JavaScript", "APIs"],
  // },
  // {
  //   title: "Understanding CSS Grid",
  //   description:
  //     "A guide to mastering CSS Grid for responsive and modern web layouts.",
  //   keywords: ["CSS Grid", "CSS", "responsive design", "web design"],
  // },
  // {
  //   title: "Data Structures in Python",
  //   description:
  //     "Explore the essential data structures in Python and how to apply them effectively.",
  //   keywords: ["Python", "data structures", "programming", "algorithms"],
  // },
];

// const test_entries = Array.from(
//   items,
//   (item) =>
//     new VaultEntry(String(item.title), String(item.description), item.keywords)
// );

const DefaultVault = new Vault();
// DefaultVault.add_entry(test_entries);

export default DefaultVault;
