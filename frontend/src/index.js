const rust = import("../pkg");

window.rust = rust;

rust.then((m) => {
  window.module = m;
});
