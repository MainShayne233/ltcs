const Elm = import("./Main.elm");
const rust = import("../pkg");

rust.then((m) => {
  window.module = m;
});

Elm.then((mod) => {
  const elm = mod.default.Elm;
  elm.Main.init({
    node: document.getElementById("elm-root"),
  });
});
