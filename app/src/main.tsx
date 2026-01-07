import type { ComponentChildren } from "preact";
import { render } from "preact";

const appRoot = document.getElementById("appRoot")!;

function Landing(): ComponentChildren {
  return (
    <code style={{ fontSize: "24px" }}>
      i'm lexi and fyi this is my website
    </code>
  );
}

render(<Landing />, appRoot);
