// import { init, on } from "./vendor/virtual-keyboard-detector.js";

// // // Start listening for virtual keyboard (dis)appearences
// init({ recentlyFocusedTimeoutDuration: 300 });
// // Handle the appearing of the virtual keyboard
// on("virtualKeyboardVisible", (document.body.style.marginBottom += "200px"));
// // Handle the disappearing of the virtual keyboard
// on("virtualKeyboardHidden", (document.body.style.marginBottom -= "200px"));


// focus events don't bubble, must use capture phase
document.body.addEventListener(
  "focus",
  (event) => {
    const target = event.target;
    switch (target.tagName) {
      case "INPUT":
      case "TEXTAREA":
      case "SELECT":
        document.body.style.marginBottom += "200px";
    }
  },
  true
);

document.body.addEventListener(
  "blur",
  (event) => {
    const target = event.target;
    switch (target.tagName) {
      case "INPUT":
      case "TEXTAREA":
      case "SELECT":
        document.body.style.marginBottom -= "200px";
    }
  },
  true
);

document.body.addEventListener(
  "click",
  () => {
    document.body.style.marginBottom -= "200px";
  },
  true
);
