import React from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
  context?: string;
  href? :string 
}
export default function SmallText({ children, className, context , href}: Props) {
 if(href){
   return (
     <a href={href}
       className={
         "leading-5 text-sm text-inherit dark:text-dark-500 cursor-pointer underline underline-offset-1 " + className
       }
     >
       {context || children}
     </a>
   );
 }
  return (
    <p
      className={
        "leading-5 text-sm text-inherit dark:text-dark-500 " + className
      }
    >
      {context || children}
    </p>
  );
}
