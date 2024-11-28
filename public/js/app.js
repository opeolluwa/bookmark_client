// // focus events don't bubble, must use capture phase
// document.body.addEventListener(
//   "focus",
//   (event) => {
//     const target = event.target;
//     switch (target.tagName) {
//       case "INPUT":
//       case "TEXTAREA":
//       case "SELECT":
//         alert("focus  in");
//         document.body.style.marginBottom += "200px";
//     }
//   },
//   true
// );

// document.body.addEventListener(
//   "focusout",
//   (event) => {
//     const target = event.target;
//     switch (target.tagName) {
//       case "INPUT":
//       case "TEXTAREA":
//       case "SELECT":
//         alert("focus out");
//         document.body.style.marginBottom -= "200px";
//     }
//   },
//   true
// );

const inputFields = document.querySelectorAll("input, textarea, select");

const handleInputFieldFocus = () => {
  // alert("focus in");
  document.body.style.marginBottom += "150px";
};

const handleInputFieldFocusOut = () => {
  // alert("focus out");
  document.body.style.marginBottom -= "150px";
};

inputFields.forEach((element) => {
  element.addEventListener("focus", handleInputFieldFocus);
  element.addEventListener("focusout", handleInputFieldFocusOut);
});
