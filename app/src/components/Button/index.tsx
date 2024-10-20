interface Props {
  children?: React.ReactNode;
  className?: string;
  href?: string;
  onClick?: () => void;
}

export default function Button({ children, className, onClick, href }: Props) {
  if (href) {
    return (
      <a
        href={href}
        onClick={onClick}
        className={"rounded px-4 py-3 block cursor-pointer " + className}
      >
        {children}
      </a>
    );
  } else {
    return (
      <button
        className={"rounded px-4 py-3 block w-full " + className}
        onClick={onClick}
      >
        {children}
      </button>
    );
  }
}
