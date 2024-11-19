import React, { useEffect, useRef, useState } from "react";

export const VisualViewport = ({
  as: Element = "div",
  children,
  style = {},
  ...props
}) => {
  const ref = useRef(null);

  const [viewport, setViewport] = useState({
    maxHeight: "100vh",
    maxWidth: "100vw",
  });

  const updateViewport = () => {
    setViewport({
      maxHeight: window.visualViewport.height,
      maxWidth: window.visualViewport.width,
    });

    window.scrollTo(0, ref.current.offsetTop);
  };

  useEffect(() => {
    if (
      typeof window !== "undefined" &&
      typeof window.visualViewport !== "undefined"
    ) {
      updateViewport();

      window.visualViewport.addEventListener("resize", updateViewport);

      return () =>
        window.visualViewport.removeEventListener("resize", updateViewport);
    }
  }, []);

  return (
    <Element ref={ref} style={{ ...style, ...viewport }} {...props}>
      {children}
    </Element>
  );
};
