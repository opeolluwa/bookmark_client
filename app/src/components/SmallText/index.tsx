import React from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
  context?: string;
  href?: string;
  onClick?: () => void;
}
export default function SmallText({
  children,
  className,
  context,
  href,
  onClick,
}: Props) {
  if (href) {
    return (
      <a
        href={href}
        onClick={() => onClick}
        className={
          "leading-5 text-sm text-inherit dark:text-dark-500 cursor-pointer underline underline-offset-1 " +
          className
        }
      >
        {context || children}
      </a>
    );
  }
  return (
    <p
      onClick={() => onClick}
      className={
        "leading-5 text-sm text-inherit dark:text-dark-500 " + className
      }
    >
      {context || children}
    </p>
  );
}
